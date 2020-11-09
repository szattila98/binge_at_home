# Binge@Home
Simple video streaming web-app for home networks. 

## Functions
It lists video resources on the main page, and the users can choose from the list. It plays videos in a video.js player.

## Setup
Setup automatically with provided *setup.sh* script. The script will output every essential files into the *dist* folder.
```
sh setup.sh
```
Alternatively setup manually as described in the front-end and back-end ReadMe-s.

## How to run
Run with the provided *start.sh* script in the *dist* folder or as described in the back-end ReadMe.
```
sh start.sh
```
The server can be run on a different port with *start.sh* by inputting the desired port number as the first argument.
```
sh start.sh ${port}
```
