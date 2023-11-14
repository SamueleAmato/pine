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

## Installation

<br>

`curl -sSL https://raw.githubusercontent.com/SamueleAmato/pine/main/install.sh | bash`

<br>

## Usage
_Pine is a tool designed for **organizing and managing** notes and notebooks. Below, you'll find information on how to get started:_

### Creating a Notebook
To create a new notebook, use the following command:
  - ```pine --create notebook --name <notebook_name>```
### Creating a Note
To create a standalone note without associating it with a notebook, use:
  - ```pine --create note --name <note_name>```
### Creating a Note in a Notebook
To create a note within a specific notebook, use:
  - ```pine --create note --tonotebook <notebook_name> --name <note_name>```
### Remove Note
To remove a note, use:
  - ```pine --remove note --name <note_name>```
### Remove a Note in a Notebook
To remove a note within a specific notebook, use:
  - ```pine --remove note --name <note_name> --innotebook <notebook_name>```
### Remove Notebook
To remove a notebook, use the following command:
  - ```pine --remove notebook --name <notebook_name>```




## To-Do List

- [X] Initialize the repository.
- [X] Add open text editor when create a note.
- [ ] SubNotebook.
- [ ] Add warning before removing note.
- [ ] Add telegram todo synch.
- [ ] Add todo list.
- [ ] add dropbox synch.
