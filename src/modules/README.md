# Webview Modules

The `modules/` directory represents the Webview-side interface for communicating with Core modules.

In the Core, "modules" refer to individual features that the Webview can interact with through commands and events. This
directory ensures alignment between the Webview and Core in terms of the structure and content of the messages
exchanged, while maintaining separation of concerns.

Each Webview module subdirectory serves the following purposes:

- **Command Invokers**: Type-safe functions for calling commands defined in Core modules.
- **Event Listeners**: Type-safe functions for responding to events emitted by Core modules.
- **Event Triggers**: Type-safe functions for emitting events to Core modules.
- **Type Definitions**: Shared types and interfaces between the Webview and Core.
