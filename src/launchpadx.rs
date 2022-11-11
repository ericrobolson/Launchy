// Heavily based on
// https://fael-downloads-prod.focusrite.com/customer/prod/s3fs-public/downloads/Launchpad%20X%20-%20Programmers%20Reference%20Manual.pdf

extern crate midir;
use crate::types::{Cell, Grid, Led, PadEvent, PadEventType};
use midir::{Ignore, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver},
};

const CELL_ROWS: usize = 9;
const CELL_COLUMNS: usize = 9;

type IONames = [&'static str; 2];
const MIDI_INPUTS: IONames = [
    "LPX MIDI",
    "MIDIIN2 (LPX MIDI)",
    //  "MIDIIN2 (Launch Control XL)",
    //   "Launch Control XL",
];
const MIDI_OUTPUTS: IONames = [
    "LPX MIDI",
    "MIDIOUT2 (LPX MIDI)",
    //   "MIDIOUT2 (Launch Control XL)",
    //   "Launch Control XL",
];

const LAUNCHPAD_INPUT: &'static str = "LAUNCHY_LaunchpadX_Input";
const LAUNCHPAD_OUTPUT: &'static str = "LAUNCHY_LaunchpadX_Output";
const LAUNCHPAD_OUTPUT_SEND: &'static str = "LAUNCHY_LaunchpadX_Output_SEND";
const LAUNCHPAD_READ_INPUT: &'static str = "LAUNCHY_LaunchpadX_Input_READ";

pub struct LaunchpadX {
    conn_out: MidiOutputConnection,
    #[allow(dead_code)]
    conn_in: MidiInputConnection<()>,
    pending_cells: Grid,
    active_cells: Grid,
    msg_outbox: Vec<PadEvent>,
    msg_state: HashMap<u8, u8>,
    msg_receiver: Receiver<MidiMsg>,
}
impl LaunchpadX {
    /// Creates a new connection to the first Launchpad.
    pub fn new() -> Self {
        let (conn_in, msg_receiver) = input_connection();
        let mut launch_pad = Self {
            conn_out: output_connection(),
            conn_in,
            pending_cells: Grid::new(CELL_ROWS, CELL_COLUMNS),
            active_cells: Grid::new(CELL_ROWS, CELL_COLUMNS),
            msg_outbox: vec![],
            msg_state: HashMap::new(),
            msg_receiver,
        };

        launch_pad.set_programmer_mode(true);

        // Reset the LEDs to be blank
        for x in 0..launch_pad.width() {
            for y in 0..launch_pad.height() {
                launch_pad.set_led(x, y, Led::Off);
            }
        }
        launch_pad.flush();

        launch_pad
    }

    /// Returns the width of the pad.
    pub fn width(&self) -> usize {
        CELL_ROWS
    }

    /// Returns the height of the pad.
    pub fn height(&self) -> usize {
        CELL_COLUMNS
    }

    /// Sets the launchpad to programmer mode.
    fn set_programmer_mode(&mut self, is_programmer_mode: bool) {
        // Set header
        let mut msg = vec![240, 0, 32, 41, 2, 12, 0];

        // Set code
        let code = match is_programmer_mode {
            true => 127,
            false => 0,
        };
        msg.push(code);

        // Finish
        msg.push(247);

        // Send
        self.conn_out.send(&msg).unwrap();
    }

    /// Queues up setting of the given LED.
    pub fn set_led(&mut self, x: usize, y: usize, state: Led) {
        self.pending_cells.set(x, y, state);
    }

    /// Flushes the colors to the Launchpad.
    pub fn flush(&mut self) {
        let msg = cells_to_sysex(&mut self.pending_cells, &mut self.active_cells);
        self.conn_out.send(&msg.to_bytes()).unwrap();
    }

    /// Polls to see if there's any event that should be handled.
    pub fn poll(&mut self) -> Option<PadEvent> {
        for msg in self.msg_receiver.try_iter() {
            let note = msg.note;
            let value = msg.velocity;
            let prev_value = self.msg_state.insert(note, value);

            self.msg_outbox.push(map_event(note, value, prev_value));
        }

        if self.msg_outbox.is_empty() {
            None
        } else {
            Some(self.msg_outbox.remove(0))
        }
    }
}

/// A struct used for communicating midi messages.
#[allow(dead_code)]
struct MidiMsg {
    code: u8,
    note: u8,
    velocity: u8,
}

#[allow(unused_variables)]
fn input_connection() -> (MidiInputConnection<()>, Receiver<MidiMsg>) {
    let mut midi_in = MidiInput::new(LAUNCHPAD_INPUT).unwrap();
    midi_in.ignore(Ignore::None);

    let input_port = {
        let mut port = None;

        for p in midi_in.ports() {
            let name = midi_in.port_name(&p).unwrap();
            if MIDI_INPUTS.contains(&name.as_str()) {
                port = Some(p);
            }

            #[cfg(feature = "debug")]
            {
                println!("{:?}", name);
            }
        }

        match port {
            Some(p) => p,
            None => todo!("Fetch input port"),
        }
    };

    let (sender, receiver) = channel();

    let conn_in = midi_in
        .connect(
            &input_port,
            LAUNCHPAD_READ_INPUT,
            move |stamp, message, _| {
                #[cfg(feature = "debug")]
                {
                    println!("{}: {:?} (len = {})", stamp, message, message.len());
                }
                if message.len() == 3 {
                    sender
                        .send(MidiMsg {
                            code: message[0],
                            note: message[1],
                            velocity: message[2],
                        })
                        .unwrap();
                }
            },
            (),
        )
        .unwrap();

    (conn_in, receiver)
}

fn output_connection() -> MidiOutputConnection {
    let midi_out = MidiOutput::new(LAUNCHPAD_OUTPUT).unwrap();
    let output_port = {
        let mut port = None;

        for p in midi_out.ports() {
            let name = midi_out.port_name(&p).unwrap();
            if MIDI_OUTPUTS.contains(&name.as_str()) {
                port = Some(p);
            }
        }

        match port {
            Some(p) => p,
            None => todo!("Fetch output port"),
        }
    };

    midi_out
        .connect(&output_port, LAUNCHPAD_OUTPUT_SEND)
        .unwrap()
}

fn map_event(note: u8, value: u8, prev_value: Option<u8>) -> PadEvent {
    let event = if value == 0 {
        PadEventType::Released
    } else {
        if prev_value.is_some() {
            PadEventType::Held { velocity: value }
        } else {
            PadEventType::Pressed { velocity: value }
        }
    };

    let x = (note % 10) - 1;
    let y = (note / 10) - 1;

    PadEvent {
        x: x as usize,
        y: y as usize,
        event,
    }
}

impl Drop for LaunchpadX {
    fn drop(&mut self) {
        self.set_programmer_mode(false);
    }
}

pub struct SysexMsg(Vec<u8>);
impl SysexMsg {
    pub fn to_bytes(self) -> Vec<u8> {
        self.0
    }
}

fn cells_to_sysex(pending_cells: &Grid, active_cells: &mut Grid) -> SysexMsg {
    // Build up message with header
    let mut msg = vec![240, 0, 32, 41, 2, 12, 3];
    for cell in active_cells.iter_mut() {
        let pending = pending_cells.get(cell.x, cell.y);
        if *cell != pending {
            write_cell_lighting_msg(&pending, &mut msg);
        }
    }

    // Stop message
    msg.push(247);

    SysexMsg(msg)
}

fn write_cell_lighting_msg(cell: &Cell, msg: &mut Vec<u8>) {
    const STATIC_COLOR: u8 = 0;
    const RGB_COLOR: u8 = 3;

    let led_type = match cell.led {
        Led::Off => STATIC_COLOR,
        Led::Rgb(_) => RGB_COLOR,
    };

    msg.push(led_type);
    msg.push(led_index(cell));

    match cell.led {
        Led::Off => msg.push(0),
        Led::Rgb((r, g, b)) => {
            msg.push(r);
            msg.push(g);
            msg.push(b);
        }
    }
}

fn led_index(cell: &Cell) -> u8 {
    let idx = (cell.y * 10 + 11) + (cell.x);
    idx as u8
}
