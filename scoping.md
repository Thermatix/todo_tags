# ToDo Tags

## Description 

A tool to manage a list of todo items that can be tagged. When you show the
list of items it will show under each item the file path of any file that
has matching tags. You tag files by stimply writing them in for example
a commen.

## User Stories

As the user I want to be able to:
 + For each Item:
   - write {tag} {description} or {tag} followed by the {description}
   - delete {tag}
   - show:
     ```Markdown
       [tag]: <Description>
         + file_path/releative/to_project/root
         + file_path/releative/to_project/root
         + file_path/releative/to_project/root
     ```
 + For a Project:
   - add {folder} {name} or {folder} or without a name
   - edit {name} meta-data (not sure what that will be yet)
   - show:
     ```Markdown
     # Project: <Project Name>
     
     ## Tags

     + Tag
     + Tag
       - Tag
     + Tag
       - Tag
       - Tag

     # Items

     + Tag: <Description>
     + Tag: <Description>
       - Tag: <Description>
         + file_path/releative/to_project/root
         + file_path/releative/to_project/root
     + Tag: <Description>
       - Tag: <Description>
       - Tag: <Description>
         + file_path/releative/to_project/root
         + file_path/releative/to_project/root
       - Tag: <Description>
         + file_path/releative/to_project/root
         + file_path/releative/to_project/root
         + file_path/releative/to_project/root
         + file_path/releative/to_project/root
     ```
   - Have each tag's file list automatically update
 + See a list of all projects
 + Manually add a project to the list of projects and have the system automatically pick it up
 + To be able to foribly close the background server

## Modules

 + ArgProcessor: process input args
 + Server: Server that runs in the background that's started on first run, in charge of accepting commands to read and write items to and from files,
   also keeps watch on any project files for filesystem changes to update tag related item paths, signal handling
 + GRPC: gRPC for communicating between the CLI and the server
 + KVDB: Key/Value File backed DB for reading and writing lists
 + Models: Projects {name: String, path: String}, Items {tag: String, description: String, paths: Vec<String> items: Vec<Item>}
 + FSWatcher: Keeps watch on project folders for file changes, sends updates to the FileProcessor
 + FProcessor: process files to check for tags and update's related Items

