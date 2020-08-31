//! A stream of audio data from the audio thread to the GUI thread

static RING_BUF_SIZE: usize = 4096;

/// Creates a new stream of audio data from the audio thread to the GUI thread
pub fn new() -> (Producer, Consumer) {
    let rb = ringbuf::RingBuffer::new(RING_BUF_SIZE);
    let (rb_prod, rb_cons) = rb.split();

    (Producer { rb_prod }, Consumer { rb_cons })
}

/// The producer of a stream of audio data from the audio thread to the GUI thread
#[allow(missing_debug_implementations)]
pub struct Producer {
    rb_prod: ringbuf::Producer<f32>,
}

impl Producer {
    /// Write new sample data into the stream
    pub fn write(&mut self, data: &[f32]) {
        let _n = self.rb_prod.push_slice(data);

        #[cfg(debug_assertions)]
        {
            if _n != data.len() {
                println!(
                    "Warning: Producer was unable to write all of its data."
                );
            }
        }
    }
}

/// The consumer of a stream of audio data from the audio thread to the GUI thread
#[allow(missing_debug_implementations)]
pub struct Consumer {
    rb_cons: ringbuf::Consumer<f32>,
}

impl Consumer {
    /// Gives immutable access to the elements contained by the ring buffer without removing them.
    ///
    /// The method takes a function f as argument. f takes two slices of ring buffer content
    /// (the second one or both of them may be empty). First slice contains older elements.
    ///
    /// The slices may not include elements pushed to the buffer by concurring producer after the method call.
    pub fn read_access<F: FnOnce(&[f32], &[f32])>(&self, f: F) {
        self.rb_cons.access(f);
    }

    /// Returns the length of readable data stored in the shared ring buffer
    pub fn len(&self) -> usize {
        self.rb_cons.len()
    }

    /// Discards the readable data stored in the shared ring buffer, freeing
    /// up space for new data to be written.
    pub fn clear(&mut self) {
        let _ = self.rb_cons.discard(self.rb_cons.capacity());
    }
}
