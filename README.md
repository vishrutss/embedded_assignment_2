# Drop
## Submitted by: Vishrut Sharma

For this assignment, I implemented the accelerometer of the MB2 chip so that when the chip is dropped it produces a sound and an exclamation mark is produced on the display.

First I referred to the following repositories to get a better understanding of the lsm303agr crate for the MB2 chip:
1) https://github.com/pdx-cs-rust-embedded/mb2-thermometer/blob/main/src/main.rs
2) https://github.com/eldruin/driver-examples/blob/master/microbit/examples/lsm303agr-accel-mb.rs

I then created separate functions to reset the board, update the board to an exclamation mark, and produce a sound. For the code to produce a sound in the MB2 chip, I referred the following repository:
https://github.com/pdx-cs-rust-embedded/hello-audio/blob/main/src/main.rs

The assignment at first looked simple but the real challenge was combining all the elements like the display, the speaker, and the accelerometer and making them work together. At first I had implemented a blocking display which was causing the sound to be very choppy and it sounded more like beeps than a continous sound. Because of this I changed it to be a non blocking display. I referred the following repository to understand how to implement a non blocking display:
https://github.com/pdx-cs-rust-embedded/mb2-grayscale/blob/main/src/main.rs

I then followed the steps given in the assignment to calculate the acceleration. I first converted the x, y, and z components of the acceleration from mG to G and then applied the formula x^2 + y^2 + z^2 < 0.25 to check if the chip was dropped. If the chip was dropped, I updated the display to an exclamation mark and produced a sound. If the board was not being dropped the display gets reset to a dot and the sound stops.

I also had a great discussion with Shrikrishna Bhat and we discussed possible methods to get the sound working on the chip and for it to be more continuous and less choppy. The sound produced is still not perfect and continuous but it is much better than my initial implementation.

Overall this was a very fun and challenging assignment which also helped me get a better understanding of the MB2 chip and the lsm303agr crate.