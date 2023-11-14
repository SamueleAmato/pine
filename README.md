<p align="center">
  <img width="200" src="images/logo.png" alt=""/>
</p>
<p align="center">
    <i>A minimal CLI note, notebook, and to-do list manager.</i><br>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/release-v0.1.0-141449" alt=""/>
  <img src="https://img.shields.io/badge/written in-rust-141449" alt=""/>
  <img src="https://img.shields.io/badge/author-SamueleAmato-141449" alt=""/>
</p>

<p align="center">
  <a href="#installation">Installation</a>
  &nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#usage">Usage</a>
  &nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#usage">Contributing</a>
  &nbsp;&nbsp;&nbsp&nbsp;&nbsp;&nbsp;
</p>

## Installation

<br>

`curl -sSL https://raw.githubusercontent.com/SamueleAmato/pine/main/install.sh | bash`

<br>

## Usage
**Pine** is a tool designed for **organizing and managing** notes and notebooks. Below, you'll find information on how to get started:

#### Creating a Notebook
To create a new notebook, use the following command:
  - ```pine --create notebook --name <notebook_name>```
#### Creating a Note
To create a standalone note without associating it with a notebook, use:
  - ```pine --create note --name <note_name>```
#### Creating a Note in a Notebook
To create a note within a specific notebook, use:
  - ```pine --create note --tonotebook <notebook_name> --name <note_name>```
#### Remove Note
To remove a note, use:
  - ```pine --remove note --name <note_name>```
#### Remove a Note in a Notebook
To remove a note within a specific notebook, use:
  - ```pine --remove note --name <note_name> --innotebook <notebook_name>```
#### Remove Notebook
To remove a notebook, use the following command:
  - ```pine --remove notebook --name <notebook_name>```
#### Moving a Note to a Notebook
To move a note that is not in any notebook to a specific notebook, use:
  - ```pine --move --name <note_name> --moveto <notebook_name>```
#### Moving a Note Within a Notebook
To move a note that is already in a notebook to another notebook, use:
- ```pine --move --name <note_name> --innotebook <source_notebook> --moveto <destination_notebook>```
#### List Notebooks
To list all notebooks, use:
  - ```pine --list notebook```
#### List Standalone Notes
To list standalone notes (not associated with any notebook), use:
  - ```pine --list note```
#### List Notes in a Notebook
To list notes within a specific notebook, use:
  - ```pine --list note --innotebook <notebook_name>```
#### Open a Standalone Note
To open a standalone note, use:
  - ```pine --open --name <note_name>```
#### Open a Note in a Notebook
To open a note within a specific notebook, use:
  - ```pine --open --name <note_name> --innotebook <notebook_name>```
#### Change the text editor
To change the text editor used to open note, use:
  -  ```pine --set-editor <editor_name>``` _es: vim/nvim, micro.
## To-Do List

- [X] Initialize the repository.
- [X] Add open text editor when create a note.
- [ ] SubNotebook.
- [ ] Add warning before removing note.
- [ ] Add telegram todo synch.
- [ ] Add todo list.
- [ ] add dropbox synch.

## Contributing      
If you encounter issues or have suggestions for [pine](https://github.com/SamueleAmato), please report them in our "Issues" section. Provide detailed information for quick resolution or implementation. Thanks for contributing! Additionally, submit pull requests to enhance the code and add new features.
