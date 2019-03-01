#[macro_use]
extern crate tokio_trace;
// Tests that macros work across various invocation syntax.
//
// These are quite repetitive, and _could_ be generated by a macro. However,
// they're compile-time tests, so I want to get line numbers etc out of
// failures, and producing them with a macro would muddy the waters a bit.

#[test]
fn span() {
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, "foo", bar = 2, baz = 3);
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, "foo", bar = 2, baz = 4,);
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, "foo");
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, "bar",);
    span!(level: tokio_trace::Level::DEBUG, "foo", bar = 2, baz = 3);
    span!(level: tokio_trace::Level::DEBUG, "foo", bar = 2, baz = 4,);
    span!(level: tokio_trace::Level::DEBUG, "foo");
    span!(level: tokio_trace::Level::DEBUG, "bar",);
    span!("foo", bar = 2, baz = 3);
    span!("foo", bar = 2, baz = 4,);
    span!("foo");
    span!("bar",);
}

#[test]
fn span_root() {
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, parent: None, "foo", bar = 2, baz = 3);
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, parent: None, "foo", bar = 2, baz = 4,);
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, parent: None, "foo");
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, parent: None, "bar",);
    span!(
        level: tokio_trace::Level::DEBUG,
        parent: None,
        "foo",
        bar = 2,
        baz = 3
    );
    span!(
        level: tokio_trace::Level::DEBUG,
        parent: None,
        "foo",
        bar = 2,
        baz = 4,
    );
    span!(level: tokio_trace::Level::DEBUG, parent: None, "foo");
    span!(level: tokio_trace::Level::DEBUG, parent: None, "bar",);
    span!(parent: None, "foo", bar = 2, baz = 3);
    span!(parent: None, "foo", bar = 2, baz = 4,);
    span!(parent: None, "foo");
    span!(parent: None, "bar",);
}

#[test]
fn span_with_parent() {
    let p = span!("im_a_parent!");
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, parent: &p, "foo", bar = 2, baz = 3);
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, parent: &p, "foo", bar = 2, baz = 4,);
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, parent: &p, "foo");
    span!(target: "foo_events", level: tokio_trace::Level::DEBUG, parent: &p, "bar",);
    span!(
        level: tokio_trace::Level::DEBUG,
        parent: &p,
        "foo",
        bar = 2,
        baz = 3
    );
    span!(
        level: tokio_trace::Level::DEBUG,
        parent: &p,
        "foo",
        bar = 2,
        baz = 4,
    );
    span!(level: tokio_trace::Level::DEBUG, parent: &p, "foo");
    span!(level: tokio_trace::Level::DEBUG, parent: &p, "bar",);
    span!(parent: &p, "foo", bar = 2, baz = 3);
    span!(parent: &p, "foo", bar = 2, baz = 4,);
    span!(parent: &p, "foo");
    span!(parent: &p, "bar",);
}

#[test]
fn event() {
    event!(tokio_trace::Level::DEBUG, foo = 3, bar = 2, baz = false);
    event!(tokio_trace::Level::DEBUG, foo = 3, bar = 3,);
    event!(tokio_trace::Level::DEBUG, "foo");
    event!(tokio_trace::Level::DEBUG, "foo: {}", 3);
    event!(tokio_trace::Level::DEBUG, { foo = 3, bar = 80 }, "baz");
    event!(tokio_trace::Level::DEBUG, { foo = 2, bar = 79 }, "baz {:?}", true);
    event!(tokio_trace::Level::DEBUG, { foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    event!(tokio_trace::Level::DEBUG, { foo = 2, bar = 78, }, "baz");
    event!(target: "foo_events", tokio_trace::Level::DEBUG, foo = 3, bar = 2, baz = false);
    event!(target: "foo_events", tokio_trace::Level::DEBUG, foo = 3, bar = 3,);
    event!(target: "foo_events", tokio_trace::Level::DEBUG, "foo");
    event!(target: "foo_events", tokio_trace::Level::DEBUG, "foo: {}", 3);
    event!(target: "foo_events", tokio_trace::Level::DEBUG, { foo = 3, bar = 80 }, "baz");
    event!(target: "foo_events", tokio_trace::Level::DEBUG, { foo = 2, bar = 79 }, "baz {:?}", true);
    event!(target: "foo_events", tokio_trace::Level::DEBUG, { foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    event!(target: "foo_events", tokio_trace::Level::DEBUG, { foo = 2, bar = 78, }, "baz");
}

#[test]
fn trace() {
    trace!(foo = 3, bar = 2, baz = false);
    trace!(foo = 3, bar = 3,);
    trace!("foo");
    trace!("foo: {}", 3);
    trace!({ foo = 3, bar = 80 }, "baz");
    trace!({ foo = 2, bar = 79 }, "baz {:?}", true);
    trace!({ foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    trace!({ foo = 2, bar = 78, }, "baz");
    trace!(target: "foo_events", foo = 3, bar = 2, baz = false);
    trace!(target: "foo_events", foo = 3, bar = 3,);
    trace!(target: "foo_events", "foo");
    trace!(target: "foo_events", "foo: {}", 3);
    trace!(target: "foo_events", { foo = 3, bar = 80 }, "baz");
    trace!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}", true);
    trace!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    trace!(target: "foo_events", { foo = 2, bar = 78, }, "baz");
}

#[test]
fn debug() {
    debug!(foo = 3, bar = 2, baz = false);
    debug!(foo = 3, bar = 3,);
    debug!("foo");
    debug!("foo: {}", 3);
    debug!({ foo = 3, bar = 80 }, "baz");
    debug!({ foo = 2, bar = 79 }, "baz {:?}", true);
    debug!({ foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    debug!({ foo = 2, bar = 78, }, "baz");
    debug!(target: "foo_events", foo = 3, bar = 2, baz = false);
    debug!(target: "foo_events", foo = 3, bar = 3,);
    debug!(target: "foo_events", "foo");
    debug!(target: "foo_events", "foo: {}", 3);
    debug!(target: "foo_events", { foo = 3, bar = 80 }, "baz");
    debug!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}", true);
    debug!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    debug!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    debug!(target: "foo_events", { foo = 2, bar = 78, }, "baz");
}

#[test]
fn info() {
    info!(foo = 3, bar = 2, baz = false);
    info!(foo = 3, bar = 3,);
    info!("foo");
    info!("foo: {}", 3);
    info!({ foo = 3, bar = 80 }, "baz");
    info!({ foo = 2, bar = 79 }, "baz {:?}", true);
    info!({ foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    info!({ foo = 2, bar = 78, }, "baz");
    info!(target: "foo_events", foo = 3, bar = 2, baz = false);
    info!(target: "foo_events", foo = 3, bar = 3,);
    info!(target: "foo_events", "foo");
    info!(target: "foo_events", "foo: {}", 3);
    info!(target: "foo_events", { foo = 3, bar = 80 }, "baz");
    info!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}", true);
    info!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    info!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    info!(target: "foo_events", { foo = 2, bar = 78, }, "baz");
}

#[test]
fn warn() {
    warn!(foo = 3, bar = 2, baz = false);
    warn!(foo = 3, bar = 3,);
    warn!("foo");
    warn!("foo: {}", 3);
    warn!({ foo = 3, bar = 80 }, "baz");
    warn!({ foo = 2, bar = 79 }, "baz {:?}", true);
    warn!({ foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    warn!({ foo = 2, bar = 78 }, "baz");
    warn!(target: "foo_events", foo = 3, bar = 2, baz = false);
    warn!(target: "foo_events", foo = 3, bar = 3,);
    warn!(target: "foo_events", "foo");
    warn!(target: "foo_events", "foo: {}", 3);
    warn!(target: "foo_events", { foo = 3, bar = 80 }, "baz");
    warn!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}", true);
    warn!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    warn!(target: "foo_events", { foo = 2, bar = 78, }, "baz");
}

#[test]
fn error() {
    error!(foo = 3, bar = 2, baz = false);
    error!(foo = 3, bar = 3,);
    error!("foo");
    error!("foo: {}", 3);
    error!({ foo = 3, bar = 80 }, "baz");
    error!({ foo = 2, bar = 79 }, "baz {:?}", true);
    error!({ foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    error!({ foo = 2, bar = 78, }, "baz");
    error!(target: "foo_events", foo = 3, bar = 2, baz = false);
    error!(target: "foo_events", foo = 3, bar = 3,);
    error!(target: "foo_events", "foo");
    error!(target: "foo_events", "foo: {}", 3);
    error!(target: "foo_events", { foo = 3, bar = 80 }, "baz");
    error!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}", true);
    error!(target: "foo_events", { foo = 2, bar = 79 }, "baz {:?}, {quux}", true, quux = false);
    error!(target: "foo_events", { foo = 2, bar = 78, }, "baz");
}
