# Voynich-term

`voynich-term` is an ASCII terminal-based chat application written using the [voynich](https://github.com/jacklund/voynich) library. Here is a screenshot:

![Screenshot of voynich-term](https://github.com/jacklund/voynich-term/blob/main/src/voynich-term-screenshot.png)

It allows you to connect to, or receive connections from, multiple `voynich` users. Each chat session is kept in a separate tab.

## Connecting to Another User

To connect to another user, you bring up the command popup using ctrl-k, and type `connect <onion-address>:<port>` to connect. On the other side, a window will pop up asking the other user if they want to accept a connection from your address; if they hit "Accept", you'll be connected.

## Help

You can bring up a help screen by typing ctrl-h.

## Key Mapping

| Key Combination | Action |
| --------------- | ------ |
| ctrl-h | Bring up the help screen |
| ctrl-k | Bring up the command input window |
| ctrl-\<left-arrow\> | Switch to the tab to the left |
| ctrl-\<right-arrow\> | Switch to the tab to the right |
| \<left-arrow\> | Move cursor left |
| \<right-arrow\> | Move cursor right |
| ctrl-u | Clear input to cursor |
| ctrl-c | Quit application |

## Commands

These are the commands that you can enter into the command window (ctrl-k):

| Command | Action |
| ------- | ------ |
| connect \<onion-address\>:\<port\> | Connect to the user at the given onion address and port |
| quit | | Quit application |

## Testing the Connection to your Onion Service

By default, the application tests whether the onion service it creates can be connected to, by connecting to it. This can take several seconds to a minute, but will verify that the onion service is in fact connectable through Tor. If you want to bypass this check, pass `--no-connection-test` on the command line.
