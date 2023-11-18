# EzEdit
This is yet another try to create text editor

EzEdit divided in client side and server side this means you can
change frontend of editor.

EzEdit using json-rpc server to handle actions and edit text.

Think about design first!
TODO:
- [ ] Implement naive prototype backend
    - [x] Text Abstraction
    - [ ] States(??):
        - [ ] Selection
        - [ ] Insertion
        - [ ] ESC/Neutral mod(just hjkl move)
    - [ ] Methods on text:
        - [ ] Insertion
        - [ ] Replacement
        - [ ] Deletation
        - [ ] Copy
        - [ ] Paste


- [ ] Implement naive minimal frontend with tauri/svelte (maybe not)

- [ ] Implement Rope B-Tree
- [ ] Unicode support

FIX:
- [ ] Asyncronous method but working with buffer is sync
- [ ] Is buffer a global state?
