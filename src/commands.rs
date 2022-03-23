use bevy::{
  prelude::*,
  app::AppExit
};


/*
 * CommandId
 *
 * A list of all the different types of commands that
 * can be issued by an admin.
 */
#[derive(PartialEq, Eq)]
enum CommandId {
  Quit,
  Status,
  Unknown
}


/*
 * CommandEvent
 *
 * A message sent by the parsing system to various
 * responder systems which are each assigned to a
 * specific command id. If the command requires
 * arguments, the parsing system will send those
 * as well.
 */
struct CommandEvent {
  pub id: CommandId,
  pub argc: u8,
  pub argv: Vec<String>
}


/*
 * parse_command
 *
 * This method can be called by any text input which
 * is designed to read in user input for commands.
 * Pass in the input in a single string called
 * 'command', as well as an event writer obtained by
 * the system this method is being called from, and
 * it will send an event which an appropriate
 * responder will handle.
 */
fn parse_command(
  command: String,
  mut event_writer: EventWriter<CommandEvent>
) {
  let id = command.trim().to_lowercase();

  match id.as_str() {
    "quit" => {
      event_writer.send(CommandEvent {
        id: CommandId::Quit,
        argc: 0,
        argv: vec![]
      });
    },
    "status" => {
      event_writer.send(CommandEvent {
        id: CommandId::Status,
        argc: 0,
        argv: vec![]
      });
    },
    _ => {
      event_writer.send(CommandEvent {
        id: CommandId::Unknown,
        argc: 0,
        argv: vec![id]
      });
    }
  }
}


/*
 * read_standard_input
 *
 * A bevy system designed to simply read from stdin
 * and pass the result to parse_command.
 */
fn read_standard_input(event_writer: EventWriter<CommandEvent>) {
  let mut command = String::new();
  std::io::stdin().read_line(&mut command).unwrap();
  parse_command(command, event_writer);
}


/*
 * command_quit
 *
 * Responds to command events which have the Quit id,
 * and will close the app.
 */
fn command_quit(
  mut command_reader: EventReader<CommandEvent>,
  mut exit_writer: EventWriter<AppExit>
) {
  for command in command_reader.iter() {
    if command.id == CommandId::Quit {
      info!("Now exiting...");
      exit_writer.send(AppExit);
    }
  }
}


/*
 * command_unknown
 *
 * Responds to command events which have the Unknown id,
 * which are sent by the parser if it fails. It will
 * simply log an error message describing what went wrong.
 */
fn command_unknown(
  mut command_reader: EventReader<CommandEvent>
) {
  for command in command_reader.iter() {
    if command.id == CommandId::Unknown {
      error!("Unknown command: {}", command.argv[0]);
    }
  }
}


/*
 * command_status
 *
 * Responds to command events which have the Status id,
 * and will send an update on server health.
 */
fn command_status (
  mut command_reader: EventReader<CommandEvent>
) {
  for command in command_reader.iter() {
    if command.id == CommandId::Status {
      info!("Server is healthy.");
    }
  }
}


/*
 * CommandPlugin
 *
 * Registers all input readers, events, and responders
 * related to commands.
 */
pub struct CommandPlugin;
impl Plugin for CommandPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<CommandEvent>()
      .add_system(read_standard_input)
      .add_system(command_quit)
      .add_system(command_status)
      .add_system(command_unknown);
  }
}
