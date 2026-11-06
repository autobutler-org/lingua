//! A standing compile runner.
//!
//! `lingua build --serve` listens on `{source root}/.lingua/runner.sock`. A later `lingua build`
//! in that root submits its jobs there. With no runner listening, the build runs in-process.
//! A failed job does not end [`Listener::run`]. `lingua build --stop` does.
//!
//! The socket path has to fit in `sockaddr_un`. On macOS that is 104 bytes including the
//! trailing NUL, and 108 on Linux.

use std::io::{ErrorKind, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};

use super::{Error, Job, Outcome, Profile, Runner};
use crate::check::{Cache, Options};

const VERSION: u8 = 1;
const JOBS: u8 = 1;
const STOP: u8 = 2;
const OUTCOMES: u8 = 3;
const STOPPED: u8 = 4;
const MAX_MESSAGE: usize = 16 * 1024 * 1024;

/// Why a standing runner could not bind, submit, or stop.
#[derive(Debug)]
pub enum ServeError {
    /// Exit 2. The message says what to do.
    Usage(String),
    /// The runner accepted the connection and then failed. Do not run the job again.
    Broken(String),
}

impl std::fmt::Display for ServeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Usage(message) | Self::Broken(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for ServeError {}

/// The unix socket [`bind`] listens on for `root`.
#[must_use]
pub fn socket_path(root: &Path) -> PathBuf {
    root.join(".lingua").join("runner.sock")
}

/// Listens for jobs until [`Listener::run`] returns.
#[derive(Debug)]
pub struct Listener {
    path: PathBuf,
    socket: UnixListener,
}

impl Listener {
    /// The socket path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Accepts jobs until a stop message. Each batch runs on `runner`.
    /// A failed job is one outcome; the loop keeps accepting.
    ///
    /// # Errors
    /// The listener itself failed. One bad client does not cause this.
    pub fn run(self, runner: &mut Runner) -> Result<(), ServeError> {
        accept_loop(&self, runner)
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Binds the socket for `root`, replacing a socket whose runner is already gone.
///
/// # Errors
/// The path is too long, the directory is not writable, or a runner is already listening.
pub fn bind(root: &Path) -> Result<Listener, ServeError> {
    let path = socket_path(root);
    ensure_socket_len(&path)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            ServeError::Usage(format!(
                "cannot create `{}`: {err}\ncheck that the source root is writable",
                parent.display()
            ))
        })?;
    }
    if path.exists() {
        match UnixStream::connect(&path) {
            Ok(_) => {
                return Err(ServeError::Usage(format!(
                    "a runner is already listening at `{}`; stop it with `lingua build --stop`",
                    path.display()
                )));
            }
            Err(_) => std::fs::remove_file(&path).map_err(|err| {
                ServeError::Usage(format!(
                    "cannot replace the stale socket at `{}`: {err}\nremove that file and run `lingua build --serve` again",
                    path.display()
                ))
            })?,
        }
    }
    match UnixListener::bind(&path) {
        Ok(socket) => Ok(Listener { path, socket }),
        Err(err) if err.kind() == ErrorKind::AddrInUse => Err(ServeError::Usage(format!(
            "a runner is already listening at `{}`; stop it with `lingua build --stop`",
            path.display()
        ))),
        Err(err) => Err(ServeError::Usage(format!(
            "cannot listen at `{}`: {err}",
            path.display()
        ))),
    }
}

/// Sends `jobs` to the runner listening in `root`.
///
/// `Ok(None)` means nothing is listening, or the job cannot be encoded, so the caller runs
/// the jobs in-process. `Ok(Some)` is the runner's outcomes, in submission order.
///
/// # Errors
/// The runner accepted the jobs and the response did not come back. The caller must not
/// run those jobs again.
pub fn submit(root: &Path, jobs: &[Job]) -> Result<Option<Vec<Outcome>>, ServeError> {
    if jobs.is_empty() || ensure_socket_len(&socket_path(root)).is_err() {
        return Ok(None);
    }
    let Some(cwd) = cwd_utf8() else {
        return Ok(None);
    };
    let Ok(body) = encode_jobs(&cwd, jobs) else {
        return Ok(None);
    };
    let mut stream = match UnixStream::connect(socket_path(root)) {
        Ok(stream) => stream,
        Err(err) if absent(&err) => return Ok(None),
        Err(err) => {
            return Err(ServeError::Broken(format!(
                "cannot reach the runner at `{}`: {err}\nstop it with `lingua build --stop`, or remove the socket and build again",
                socket_path(root).display()
            )));
        }
    };
    write_frame(&mut stream, &body).map_err(|err| {
        ServeError::Broken(format!(
            "the standing runner accepted the connection but not the job: {err}\nstop it with `lingua build --stop` and start it again"
        ))
    })?;
    let body = read_message(&mut stream).map_err(|err| {
        ServeError::Broken(format!(
            "the standing runner accepted the job but the response did not arrive: {err}\nthe executable may already have been written; check the output path. If this keeps happening, `lingua build --stop` and start the runner again"
        ))
    })?;
    decode_outcomes(&body).map(Some).map_err(|err| {
        ServeError::Broken(format!(
            "the standing runner sent a response lingua does not understand ({err}); stop it with `lingua build --stop` and start it again"
        ))
    })
}

/// Tells the runner in `root` to exit and remove its socket.
///
/// `Ok(false)` means nothing is listening.
///
/// # Errors
/// A runner was there and the stop did not finish cleanly.
pub fn stop(root: &Path) -> Result<bool, ServeError> {
    let path = socket_path(root);
    if ensure_socket_len(&path).is_err() {
        return Ok(false);
    }
    let mut stream = match UnixStream::connect(&path) {
        Ok(stream) => stream,
        Err(err) if absent(&err) => return Ok(false),
        Err(err) => {
            return Err(ServeError::Broken(format!(
                "cannot reach the runner at `{}`: {err}\nremove the socket if that runner is already gone",
                path.display()
            )));
        }
    };
    write_frame(&mut stream, &encode_simple(STOP)).map_err(|err| {
        ServeError::Broken(format!(
            "could not tell the runner at `{}` to stop: {err}",
            path.display()
        ))
    })?;
    match read_message(&mut stream) {
        Ok(body) if body.first() == Some(&VERSION) && body.get(1) == Some(&STOPPED) => Ok(true),
        Ok(_) => Err(ServeError::Broken(
            "the standing runner sent a response lingua does not understand; stop it with `lingua build --stop` and start it again"
                .to_owned(),
        )),
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => Ok(true),
        Err(err) => Err(ServeError::Broken(format!(
            "the runner at `{}` did not confirm it stopped: {err}",
            path.display()
        ))),
    }
}

fn accept_loop(listener: &Listener, runner: &mut Runner) -> Result<(), ServeError> {
    loop {
        let mut stream = accept(&listener.socket)?;
        if let Ok(true) = exchange(&mut stream, runner) {
            return Ok(());
        }
    }
}

fn accept(listener: &UnixListener) -> Result<UnixStream, ServeError> {
    loop {
        match listener.accept() {
            Ok((stream, _)) => return Ok(stream),
            Err(err) if err.kind() == ErrorKind::Interrupted => {}
            Err(err) => {
                return Err(ServeError::Broken(format!(
                    "the standing runner stopped accepting jobs: {err}\nstart it again with `lingua build --serve`"
                )));
            }
        }
    }
}

/// `Ok(true)` is a stop. A client error does not stop the runner.
fn exchange(stream: &mut UnixStream, runner: &mut Runner) -> Result<bool, ()> {
    let body = read_message(stream).map_err(|_| ())?;
    match decode_request(&body) {
        Ok(Request::Stop) => {
            let _ = write_frame(stream, &encode_simple(STOPPED));
            Ok(true)
        }
        Ok(Request::Jobs { cwd, jobs }) => {
            let outcomes = run_jobs(runner, &cwd, jobs);
            write_frame(stream, &encode_outcomes(&outcomes)).map_err(|_| ())?;
            Ok(false)
        }
        Err(()) => Err(()),
    }
}

fn run_jobs(runner: &mut Runner, cwd: &str, jobs: Vec<Job>) -> Vec<Outcome> {
    let previous = std::env::current_dir().ok();
    if let Err(err) = std::env::set_current_dir(cwd) {
        return jobs
            .into_iter()
            .map(|_| {
                Outcome::Failed(Error::Usage(format!(
                    "cannot enter `{cwd}`: {err}\ncheck that the directory still exists"
                )))
            })
            .collect();
    }
    let _restore = previous.map(CwdGuard);
    for job in jobs {
        runner.submit(job);
    }
    runner.run()
}

struct CwdGuard(PathBuf);

impl Drop for CwdGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.0);
    }
}

enum Request {
    Stop,
    Jobs { cwd: String, jobs: Vec<Job> },
}

fn absent(err: &std::io::Error) -> bool {
    // `InvalidInput` is the std error when the socket path does not fit in `sockaddr_un`.
    // Nothing can be listening there, so the build runs in-process.
    matches!(
        err.kind(),
        ErrorKind::NotFound | ErrorKind::ConnectionRefused | ErrorKind::InvalidInput
    )
}

fn cwd_utf8() -> Option<String> {
    std::env::current_dir()
        .ok()?
        .into_os_string()
        .into_string()
        .ok()
}

fn ensure_socket_len(path: &Path) -> Result<(), ServeError> {
    // `sun_path` includes a trailing NUL. macOS allows 104 bytes, Linux 108.
    let limit = if cfg!(target_os = "macos") { 103 } else { 107 };
    if path.as_os_str().as_bytes().len() > limit {
        return Err(ServeError::Usage(format!(
            "the source root is too deep for a unix socket at `{}`; move the package to a shorter directory",
            path.display()
        )));
    }
    Ok(())
}

fn encode_simple(kind: u8) -> Vec<u8> {
    vec![VERSION, kind]
}

fn encode_jobs(cwd: &str, jobs: &[Job]) -> Result<Vec<u8>, ServeError> {
    let mut body = encode_simple(JOBS);
    push_str(&mut body, cwd)?;
    push_u32(&mut body, jobs.len())?;
    for job in jobs {
        body.push(match job.profile {
            Profile::Debug => 0,
            Profile::Release => 1,
        });
        body.push(match job.options.cache {
            Cache::Default => 0,
            Cache::Refresh => 1,
            Cache::Offline => 2,
        });
        match &job.options.source_root {
            Some(root) => push_str(&mut body, path_text(root)?)?,
            None => push_str(&mut body, "")?,
        }
        push_str(&mut body, path_text(&job.output)?)?;
        push_u32(&mut body, job.paths.len())?;
        for path in &job.paths {
            push_str(&mut body, path_text(path)?)?;
        }
    }
    if body.len() > MAX_MESSAGE {
        return Err(ServeError::Usage(
            "this job is too large to send to the standing runner".to_owned(),
        ));
    }
    Ok(body)
}

fn encode_outcomes(outcomes: &[Outcome]) -> Vec<u8> {
    let mut body = encode_simple(OUTCOMES);
    let count = u32::try_from(outcomes.len()).unwrap_or(u32::MAX);
    body.extend(count.to_le_bytes());
    for outcome in outcomes {
        match outcome {
            Outcome::Wrote(path) => {
                body.push(0);
                push_str_lossy(&mut body, path);
            }
            Outcome::Failed(Error::Usage(message)) => {
                body.push(1);
                push_str_lossy_text(&mut body, message);
            }
            Outcome::Failed(Error::Diagnostics(diagnostics)) => {
                body.push(2);
                let text: String = diagnostics.iter().map(ToString::to_string).collect();
                push_str_lossy_text(&mut body, &text);
            }
            Outcome::Failed(Error::Rendered(text)) => {
                body.push(2);
                push_str_lossy_text(&mut body, text);
            }
            Outcome::Failed(Error::Internal(message)) => {
                body.push(3);
                push_str_lossy_text(&mut body, message);
            }
            Outcome::Unwritten { path, error } => {
                body.push(4);
                push_str_lossy(&mut body, path);
                push_str_lossy_text(&mut body, error);
            }
        }
    }
    body
}

fn decode_request(body: &[u8]) -> Result<Request, ()> {
    let mut reader = Reader::new(body);
    let version = reader.u8().ok_or(())?;
    let kind = reader.u8().ok_or(())?;
    if version != VERSION {
        return Err(());
    }
    match kind {
        STOP if reader.done() => Ok(Request::Stop),
        JOBS => {
            let cwd = reader.string().ok_or(())?;
            let count = reader.u32().ok_or(())?;
            if count > 10_000 {
                return Err(());
            }
            let mut jobs = Vec::with_capacity(usize_from_u32(count));
            for _ in 0..count {
                jobs.push(reader.job().ok_or(())?);
            }
            if reader.done() {
                Ok(Request::Jobs { cwd, jobs })
            } else {
                Err(())
            }
        }
        _ => Err(()),
    }
}

fn decode_outcomes(body: &[u8]) -> Result<Vec<Outcome>, String> {
    let mut reader = Reader::new(body);
    let version = reader.u8().ok_or_else(|| "short response".to_owned())?;
    let kind = reader.u8().ok_or_else(|| "short response".to_owned())?;
    if version != VERSION || kind != OUTCOMES {
        return Err("wrong response".to_owned());
    }
    let count = reader.u32().ok_or_else(|| "short response".to_owned())?;
    if count > 10_000 {
        return Err("too many outcomes".to_owned());
    }
    let mut outcomes = Vec::with_capacity(usize_from_u32(count));
    for _ in 0..count {
        outcomes.push(reader.outcome().ok_or_else(|| "short outcome".to_owned())?);
    }
    if reader.done() {
        Ok(outcomes)
    } else {
        Err("trailing bytes".to_owned())
    }
}

struct Reader<'a> {
    buf: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, at: 0 }
    }

    fn done(&self) -> bool {
        self.at == self.buf.len()
    }

    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.at.checked_add(n)?;
        if end > self.buf.len() {
            return None;
        }
        let out = &self.buf[self.at..end];
        self.at = end;
        Some(out)
    }

    fn u8(&mut self) -> Option<u8> {
        self.take(1).map(|bytes| bytes[0])
    }

    fn u32(&mut self) -> Option<u32> {
        let bytes = self.take(4)?;
        Some(u32::from_le_bytes(bytes.try_into().ok()?))
    }

    fn string(&mut self) -> Option<String> {
        let len = usize_from_u32(self.u32()?);
        let bytes = self.take(len)?;
        String::from_utf8(bytes.to_vec()).ok()
    }

    fn path(&mut self) -> Option<PathBuf> {
        self.string().map(PathBuf::from)
    }

    fn job(&mut self) -> Option<Job> {
        let profile = match self.u8()? {
            0 => Profile::Debug,
            1 => Profile::Release,
            _ => return None,
        };
        let cache = match self.u8()? {
            0 => Cache::Default,
            1 => Cache::Refresh,
            2 => Cache::Offline,
            _ => return None,
        };
        let source_root = self.string()?;
        let output = self.path()?;
        let count = self.u32()?;
        if count > 10_000 {
            return None;
        }
        let mut paths = Vec::with_capacity(usize_from_u32(count));
        for _ in 0..count {
            paths.push(self.path()?);
        }
        let options = Options {
            source_root: if source_root.is_empty() {
                None
            } else {
                Some(PathBuf::from(source_root))
            },
            cache,
        };
        Some(Job::new(paths, options, profile, output))
    }

    fn outcome(&mut self) -> Option<Outcome> {
        match self.u8()? {
            0 => Some(Outcome::Wrote(self.path()?)),
            1 => Some(Outcome::Failed(Error::Usage(self.string()?))),
            2 => Some(Outcome::Failed(Error::Rendered(self.string()?))),
            3 => Some(Outcome::Failed(Error::Internal(self.string()?))),
            4 => Some(Outcome::Unwritten {
                path: self.path()?,
                error: self.string()?,
            }),
            _ => None,
        }
    }
}

fn usize_from_u32(value: u32) -> usize {
    usize::try_from(value).unwrap_or(usize::MAX)
}

fn path_text(path: &Path) -> Result<&str, ServeError> {
    path.to_str().ok_or_else(|| {
        ServeError::Usage(format!(
            "path `{}` is not utf-8; the standing runner needs a utf-8 path",
            path.display()
        ))
    })
}

fn push_u32(out: &mut Vec<u8>, value: usize) -> Result<(), ServeError> {
    let value = u32::try_from(value).map_err(|_| {
        ServeError::Usage("this job is too large to send to the standing runner".to_owned())
    })?;
    out.extend(value.to_le_bytes());
    Ok(())
}

fn push_str(out: &mut Vec<u8>, text: &str) -> Result<(), ServeError> {
    push_u32(out, text.len())?;
    out.extend(text.as_bytes());
    Ok(())
}

fn push_str_lossy(out: &mut Vec<u8>, path: &Path) {
    let text = path.to_string_lossy();
    push_str_lossy_text(out, &text);
}

fn push_str_lossy_text(out: &mut Vec<u8>, text: &str) {
    let len = u32::try_from(text.len()).unwrap_or(u32::MAX);
    out.extend(len.to_le_bytes());
    let bytes = text.as_bytes();
    out.extend(&bytes[..bytes.len().min(usize_from_u32(len))]);
}

fn write_frame(stream: &mut UnixStream, body: &[u8]) -> std::io::Result<()> {
    let len = u32::try_from(body.len())
        .map_err(|_| std::io::Error::new(ErrorKind::InvalidData, "message is too long"))?;
    stream.write_all(&len.to_le_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}

fn read_message(stream: &mut UnixStream) -> std::io::Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let len = usize_from_u32(u32::from_le_bytes(len_buf));
    if len > MAX_MESSAGE {
        return Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "message is too long",
        ));
    }
    let mut body = vec![0u8; len];
    stream.read_exact(&mut body)?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::PathBuf;

    use super::{
        ErrorKind, Job, MAX_MESSAGE, Outcome, Profile, Request, ServeError, bind, decode_outcomes,
        decode_request, encode_jobs, encode_outcomes, read_message, socket_path, stop, submit,
        write_frame,
    };
    use crate::build::Error;
    use crate::check::{Cache, Diagnostic, Options};

    struct Tmp(PathBuf);

    impl Drop for Tmp {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn temp(name: &str) -> Tmp {
        let dir = PathBuf::from(format!("/tmp/lingua-s-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Tmp(dir)
    }

    #[test]
    fn jobs_and_outcomes_round_trip() {
        let job = Job::new(
            vec![PathBuf::from("hello.lf")],
            Options {
                source_root: Some(PathBuf::from("root")),
                cache: Cache::Offline,
            },
            Profile::Release,
            "lingua-out-release",
        );
        let body = encode_jobs("/tmp/work", &[job]).unwrap();
        let Request::Jobs { cwd, jobs } = decode_request(&body).unwrap() else {
            panic!("not a job request");
        };
        assert_eq!(cwd, "/tmp/work");
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].profile, Profile::Release);
        assert_eq!(jobs[0].options.cache, Cache::Offline);
        assert_eq!(
            jobs[0].options.source_root.as_deref(),
            Some(std::path::Path::new("root"))
        );
        assert_eq!(jobs[0].paths, vec![PathBuf::from("hello.lf")]);
        assert_eq!(jobs[0].output, PathBuf::from("lingua-out-release"));

        let diagnostic = Diagnostic::new("L020", "expected `Text`", "count.lf");
        let rendered = diagnostic.to_string();
        let outcomes = vec![
            Outcome::Wrote(PathBuf::from("lingua-out")),
            Outcome::Failed(Error::Usage("nope".to_owned())),
            Outcome::Failed(Error::Diagnostics(vec![diagnostic])),
            Outcome::Failed(Error::Internal("bug".to_owned())),
            Outcome::Unwritten {
                path: PathBuf::from("lingua-out"),
                error: "disk".to_owned(),
            },
        ];
        let back = decode_outcomes(&encode_outcomes(&outcomes)).unwrap();
        assert_eq!(
            back,
            vec![
                Outcome::Wrote(PathBuf::from("lingua-out")),
                Outcome::Failed(Error::Usage("nope".to_owned())),
                Outcome::Failed(Error::Rendered(rendered)),
                Outcome::Failed(Error::Internal("bug".to_owned())),
                Outcome::Unwritten {
                    path: PathBuf::from("lingua-out"),
                    error: "disk".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn a_huge_length_is_rejected_before_allocating_it() {
        let (mut writer, mut reader) = std::os::unix::net::UnixStream::pair().unwrap();
        writer.write_all(&u32::MAX.to_le_bytes()).unwrap();
        let err = read_message(&mut reader).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(MAX_MESSAGE < usize::try_from(u32::MAX).unwrap_or(usize::MAX));
    }

    #[test]
    fn frames_round_trip_across_a_socket_pair() {
        let (mut writer, mut reader) = std::os::unix::net::UnixStream::pair().unwrap();
        let body = encode_jobs("/tmp/work", &[]).unwrap();
        // An empty job list is still a legal body; the caller does not submit one.
        assert!(matches!(decode_request(&body), Ok(Request::Jobs { .. })));
        write_frame(&mut writer, &body).unwrap();
        assert_eq!(read_message(&mut reader).unwrap(), body);
    }

    #[test]
    fn a_stale_socket_is_replaced_and_a_second_bind_is_refused() {
        let dir = temp("socket");
        let path = socket_path(&dir.0);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, b"stale").unwrap();
        let listener = bind(&dir.0).unwrap();
        assert_eq!(listener.path(), path.as_path());
        let err = bind(&dir.0).unwrap_err();
        let ServeError::Usage(message) = err else {
            panic!("expected a usage error");
        };
        assert!(message.contains("already listening"), "{message}");
        assert!(message.contains("lingua build --stop"), "{message}");
        drop(listener);
        assert!(!path.exists());
        assert!(matches!(stop(&dir.0), Ok(false)));
    }

    #[test]
    fn a_deep_root_is_refused_before_bind() {
        let mut deep = PathBuf::from("/tmp");
        while socket_path(&deep).as_os_str().len() < 120 {
            deep.push("directory-name");
        }
        let err = bind(&deep).unwrap_err();
        let ServeError::Usage(message) = err else {
            panic!("expected a usage error");
        };
        assert!(message.contains("too deep"), "{message}");
        let job = Job::new(
            vec![deep.join("hello.lf")],
            Options::default(),
            Profile::Debug,
            "lingua-out",
        );
        assert!(matches!(submit(&deep, &[job]), Ok(None)));
        assert!(matches!(stop(&deep), Ok(false)));
    }
}
