use super::*;

/// Queueing and draining methods for `WindowCommandQueue`.
impl WindowCommandQueue {
    /// Pushes a window command onto the queue.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable command queue.
    /// - `WindowCommand` - Command to enqueue.
    pub fn push(&mut self, command: WindowCommand) {
        self.get_mut_commands().push(command);
    }

    /// Drains all queued commands in FIFO order.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable command queue.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowCommand>` - Drained commands.
    pub fn drain(&mut self) -> Vec<WindowCommand> {
        take(self.get_mut_commands())
    }

    /// Returns true when no window commands are pending.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Command queue reference.
    ///
    /// # Returns
    ///
    /// - `bool` - True when empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.get_commands().is_empty()
    }
}
