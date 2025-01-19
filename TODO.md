# TODO

## Background Task
> High priority

Make the app launch in the background and be always ready to input text.

Help:
- https://github.com/iced-rs/iced/issues/718
- https://discourse.iced.rs/t/how-to-map-control-s-to-a-keyboard-event/781

## Settings
> Medium-low priority

Add a settings JSON or something to be able to set some stuff like different keybindings and where
the text editor saves files

## CLI support
> Medium priority

Add CLI support, meaning you can actually open and close the UI from the CLI and pass arguments to it.
I don't think I will actually create a CLI editor.

## Editor modifications
> Medium-high priority

Make it so that the editor opens without creating an underlying file.
Ask for a file name when saving and be able to create one or not.
Auto-generate file names based on date and time.

## Create release pipeline
> Medium-high prioity

Create a GitHub workflow to release the app to Windows, macOS and Linux.
Only 64 bit systems will be supported, no support for raspberry pi or such.

## Add tests
> Low  priority

Add a test framework to track possible issues
