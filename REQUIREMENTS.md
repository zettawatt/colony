# Functional Requirements

This file outlines the functional requirements of Colony for the Impossible Futures challenge. These are the minimum features required for the product to be considered complete, prioritized for time and resource constraints.

## Milestone 1 - MVP

### Phase 1 - Initial GUI and Configuration

- [x] Initial GUI blackbox in Slint UI
- [ ] Add text number to the initial 12 word seed phrase lineedit widgets to indicate the word number (somewhat ambiguous today)
- [ ] Change timeout from seconds to minutes
- [ ] Implement timeout as a function in Rust using tokio instead of the slint mechanism since it seems to be buggy
- [ ] Add option to automatically update pod cache on startup

### Phase 2 - Pod Generation and Management

- [ ] On initial setup after clicking finish, generate first derived public key, check Autonomi if there is a file there, download, and check if it is a pod
- [ ] If something found, continue incrementing through derived public keys until no more pods are found
- [ ] If download fails for some reason, rerun
- [ ] If nothing found at first masterkey address, generate pod on initial setup using the first derived key from the master address
- [ ] Save pod locally in the data directory in a newly created 'pods' subdirectory and using the public address as the file name
- [ ] Get quote from Autonomi network on price to publish new pod
- [ ] Add callback to an 'upload' button on the 'Upload' tab and create async function to publish to Autonomi
- [ ] Write an update to the pod and indicate to the user that there is a difference between what is written and what has been published
- [ ] Write an update to the pod and publish to Autonomi with the 'upload' button and clear out the GUI difference indicator
- [ ] Add secondary pod address from another account to this pod and write recursive function to traverse all pods and download them from Autonomi to the data 'pods' subdirectory
- [ ] Write an update to a user's pod and add functionality to check that there is a difference made between the cached version (last downloaded from Autonomi) and the local copy
- [ ] Write an update to the user's pod on computer 1. Write a different update to the same pod on computer 2 and push to Autonomi. Update the cache on computer 1 and have a pop up be displayed to the user to pick which version they want to keep.
- [ ] Add the oxigraph RDF library and add a function to parse the pod files in the cache to build up an in memory database for querying
- [ ] Add a button to 'update' to redownload all pods and call the RDF parsing function
- [ ] If a pod file is encountered that throws an exception (has syntax errors or isn't a pod at all), process whatever can be processed from it and move on

### Phase 3 - File Upload

- [ ] Create file size text widget, file type dropdown, and file name lineedit fields in the upload tab
- [ ] Implement file picker utility in the GUI to select a file on the local disk. On selection, fill in the LineEdit widget with the path
- [ ] When uploading a file from the disk, grab the file size information and fill in this attribute.
- [ ] Implement Autonomi address check in the file upload LineEdit widget to enable linking to an existing file on the network. 
- [ ] When uploading content to an existing Autonomi address, check if file exists. If no content found at the address, throw out an error message in text below the file upload LineEdit widget. Else download the file to get the size and fill in this attribute
- [ ] When attributes are updated, save the updates to the local copy of the pod in RDF Turtle format
- [ ] When updates made, indicate that the pods have changes to upload ane enable the 'upload' button
- [ ] When upload button clicked, get quote from Autonomi network and display a button to pay the cost and upload the files and pod updates. Upload the files first, get the resultant address, update the pod with the final address to the file, and upload the pod updates
- [ ] Gracefully handle scenarios where Colony is closed at the different stages of preparing for and going through the upload process
- [ ] When pod scratchpad would overflow due to new file being uploaded, create a new pod scratchpad, write address to new pod scratchpad at the bottom of the original pod scratchpad, and fill in new data to this new pod
- [ ] EXTRA when new pods are added, refactor the top level pod to always list the pod addresses first to enable multi-threading pod downloads

### Phase 4 - Search

- [ ] Add callback to search button and the 'accept' action on the search lineedit widget
- [ ] Search callback would issue a SPARQL query for that term on the 'name' field in the database
- [ ] Add an 'advanced search' button that when selected replaces the search bar with a text entry widget to enable issuing a raw SPARQL query
- [ ] When search performed, list all files returned on the bottom left side of the window and populate with the appropriate name and address attributes
- [ ] Selecting a file entry will display the information about the file on the bottom right hand side and change the color of the file icon to 'selected'

### Phase 5 - File Download

- [ ] When the file is selected, clicking on the download icon on the right of the widget will start downloding the file to the downloads directory specified in the configuration file. The download icon will change to 'downloading'
- [ ] On the downloads tab display all downloads in process in the top section of the screen. Display the progress of the download as a bar and update as the file size increases
- [ ] List all completed downloads in the bottom left rectangle. Only filenames in the downloads directory that match a file in the cached pod data will be counted here
- [ ] When a download is selected, list information about that file in the bottom right rectangle with the same attributes as found on the upload tab
- [ ] Add ability to write a comment attribute or change other information about this file in the bottom right rectangle (metadata information section)
- [ ] If metadata information is updated on the downloads page, store that in the local pod file and indicate that the pod was updated. On the uploads page enable uploading the pod update to Autonomi
