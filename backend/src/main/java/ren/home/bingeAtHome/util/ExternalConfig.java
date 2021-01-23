package ren.home.bingeAtHome.util;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.Properties;
import java.util.regex.Pattern;

/**
 * Loads configuration data for the application.
 *
 * @author Attila Szőke
 */
public class ExternalConfig {

    private static final String VIDEO_STORE_PATH_PROP = "bingeAtHome.video.store.path";
    private static final String VALID_EXTENSIONS_PROP = "bingeAtHome.video.validExtensions";
    private static final String DEFAULT_VIDEOS_PATH = "." + File.separator + "videos";
    private static final String DEFAULT_EXTENSIONS = "mp4,webm";
    public static final String CONFIG_FILE = "." + File.separator + "config.properties";

    public static String VIDEO_STORE_PATH;
    public static String[] VALID_EXTENSIONS;
    public static String METADATA_STORE_PATH;
    public static String IMAGE_STORE_PATH;
    public static String TRACK_STORE_PATH;

    /**
     * Initializes the configuration variables.
     */
    public static void init() {
        Properties props = new Properties();
        File propsFile = new File(CONFIG_FILE);
        if (propsFile.exists()) {
            // Reading of properties from existing props file
            try {
                props.load(new FileInputStream(CONFIG_FILE));
                VIDEO_STORE_PATH = props.getProperty(VIDEO_STORE_PATH_PROP);
                String extProp = props.getProperty(VALID_EXTENSIONS_PROP);
                if (!Pattern.matches("^[a-zA-Z1-9]{3,4}(,*[a-zA-Z1-9]{3,4})*$", extProp)) {
                    throw new RuntimeException(
                            "APPLICATION FAILED TO START! REASON: Valid extensions should be supplied separated with commas in the config file!");
                }
                VALID_EXTENSIONS = extProp.split(",");
            } catch (IOException e) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: Config file cannot be loaded!");
            } catch (NullPointerException e) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: Required config property missing! Provide it or delete config file to generate new properties!");
            }
            if (VIDEO_STORE_PATH == null || VIDEO_STORE_PATH.isEmpty()) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: Valid video store path should be supplied in config!");
            }
        } else {
            // Creation of properties file in case it does not exist
            props.setProperty(VIDEO_STORE_PATH_PROP, DEFAULT_VIDEOS_PATH);
            props.setProperty(VALID_EXTENSIONS_PROP, DEFAULT_EXTENSIONS);
            try {
                props.store(new FileOutputStream(propsFile), "Automatically generated configuration properties.");
            } catch (IOException e) {
                throw new RuntimeException(
                        "APPLICATION FAILED TO START! REASON: Config file cannot be created!");
            }
            VIDEO_STORE_PATH = props.getProperty(VIDEO_STORE_PATH_PROP);
            VALID_EXTENSIONS = props.getProperty(VALID_EXTENSIONS_PROP).split(",");
        }
        // Creation of necessary folders
        File videoFolder = new File(VIDEO_STORE_PATH);
        if (!videoFolder.exists() && !videoFolder.mkdirs()) {
            throw new RuntimeException(
                    "APPLICATION FAILED TO START! REASON: video store directory cannot be created!");
        }
        if (!videoFolder.isDirectory()) {
            throw new RuntimeException(
                    "APPLICATION FAILED TO START! REASON: video store directory is not a directory!");
        }
        METADATA_STORE_PATH = VIDEO_STORE_PATH + File.separator + "metadata";
        File metadataFolder = new File(METADATA_STORE_PATH);
        if (!metadataFolder.exists() && !metadataFolder.mkdirs()) {
            throw new RuntimeException(
                    "APPLICATION FAILED TO START! REASON: metadata store directory cannot be created!");
        }
        IMAGE_STORE_PATH = VIDEO_STORE_PATH + File.separator + "images";
        File imageFolder = new File(IMAGE_STORE_PATH);
        if (!imageFolder.exists() && !imageFolder.mkdirs()) {
            throw new RuntimeException(
                    "APPLICATION FAILED TO START! REASON: image store directory cannot be created!");
        }
        TRACK_STORE_PATH = VIDEO_STORE_PATH + File.separator + "tracks";
        File captionFolder = new File(TRACK_STORE_PATH);
        if (!captionFolder.exists() && !captionFolder.mkdirs()) {
            throw new RuntimeException(
                    "APPLICATION FAILED TO START! REASON: caption store directory cannot be created!");
        }
    }

    /**
     * Initializes the configuration variables. Should only be used in a test environment.
     *
     * @param tempDir the test environment temporary directory
     */
    public static void test_init(File tempDir) {
        VIDEO_STORE_PATH = tempDir.getAbsolutePath() + File.separator + "videos";
        VALID_EXTENSIONS = DEFAULT_EXTENSIONS.split(",");
        METADATA_STORE_PATH = VIDEO_STORE_PATH + File.separator + "metadata";
        IMAGE_STORE_PATH = VIDEO_STORE_PATH + File.separator + "images";
        TRACK_STORE_PATH = VIDEO_STORE_PATH + File.separator + "tracks";

        new File(ExternalConfig.VIDEO_STORE_PATH).mkdir();
        new File(ExternalConfig.METADATA_STORE_PATH).mkdir();
        new File(ExternalConfig.IMAGE_STORE_PATH).mkdir();
        new File(ExternalConfig.TRACK_STORE_PATH).mkdir();
    }
}
