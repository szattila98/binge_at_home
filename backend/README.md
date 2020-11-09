# Binge@Home back-end

## Requirements
- Java 8
- Maven 3.6.0

## Project setup
Copy the *dist* folder's content from the front-end project into the back-end's *src/main/resources/static* folder.
Then run the command below. After it is done, the jar file will be in the *target* folder. 
```
mvn clean package
```
Run the jar.
```
java -jar ${packaged.jar}
```
It will generate its configuration properties, and the default video folder on the first run.
The default folder is *./videos*, and the app will serve the *mp4* and *webm* videos in it by default.
The config can be changed as desired. If the config was changed, restart is needed for the changes to take effect.
Configuration is checked on start-up and any error will be logged, and the application will terminate.
The app runs on the 80 port by default. To run on a different port.
```
java -jar ${packaged.jar} --server.port=${different_port}
```