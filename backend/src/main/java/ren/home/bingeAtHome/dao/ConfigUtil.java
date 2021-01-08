package ren.home.bingeAtHome.dao;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.Properties;
import java.util.regex.Pattern;

public class ConfigUtil {

    private static final String VIDEO_STORE_PATH_PROP = "bingeAtHome.video.store.path";
    private static final String VALID_EXTENSIONS_PROP = "bingeAtHome.video.validExtensions";
    private static final String DEFAULT_VIDEOS_PATH = "./videos";
    private static final String DEFAULT_EXTENSIONS = "mp4,webm";
    private static final String CONFIG_FILE = "./config.properties";

    public static String videoStorePath;
    public static String[] validExtensions;
    public static String metadataStorePath;

    public static void init() {
        Properties props = new Properties();
        File propsFile = new File(CONFIG_FILE);
        if (propsFile.exists()) {
            try {
                props.load(new FileInputStream(CONFIG_FILE));
                videoStorePath = props.getProperty(VIDEO_STORE_PATH_PROP);
                String extProp = props.getProperty(VALID_EXTENSIONS_PROP);
                if (!Pattern.matches("^[a-zA-Z1-9]{3,4}(,*[a-zA-Z1-9]{3,4})*$", extProp)) {
                    throw new RuntimeException(
                            "APPLICATION FAILED TO START! REASON: Valid extensions should be supplied separated with commas in the config file!");
                }
                validExtensions = extProp.split(",");
            } catch (IOException e) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: Config file cannot be loaded!");
            } catch (NullPointerException e) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: Required config property missing! Provide it or delete config file to generate new properties!");
            }
            if (videoStorePath == null || videoStorePath.isEmpty()) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: Valid video store path should be supplied in config!");
            }
            File videoStoreDir = new File(videoStorePath);
            if (!videoStoreDir.exists()) {
                if (!videoStoreDir.mkdirs()) {
                    throw new RuntimeException(
                            "APPLICATION FAILED TO START! REASON: video store directory cannot be created!");
                }
            }
            if (!videoStoreDir.isDirectory()) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: video store directory is not a directory!");
            }
        } else {
            props.setProperty(VIDEO_STORE_PATH_PROP, DEFAULT_VIDEOS_PATH);
            props.setProperty(VALID_EXTENSIONS_PROP, DEFAULT_EXTENSIONS);
            try {
                props.store(new FileOutputStream(propsFile), "Automatically generated configuration properties.");
            } catch (IOException e) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: Config file cannot be created!");
            }
            videoStorePath = props.getProperty(VIDEO_STORE_PATH_PROP);
            validExtensions = props.getProperty(VALID_EXTENSIONS_PROP).split(",");
        }
        File videoFolder = new File(videoStorePath);
        if (!videoFolder.exists()) {
            if (!videoFolder.mkdirs()) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: video store directory cannot be created!");
            }
        }
        metadataStorePath = videoStorePath + "/" + "metadata";
        File metadataFolder = new File(metadataStorePath);
        if (!metadataFolder.exists()) {
            if (!metadataFolder.mkdirs()) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: metadata store directory cannot be created!");
            }
        }
    }
}
