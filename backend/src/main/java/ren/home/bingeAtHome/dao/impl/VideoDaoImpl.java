package ren.home.bingeAtHome.dao.impl;

import org.apache.commons.io.FileUtils;
import org.springframework.core.io.UrlResource;
import org.springframework.stereotype.Component;
import ren.home.bingeAtHome.dao.VideoDao;

import javax.annotation.PostConstruct;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.net.MalformedURLException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.Properties;
import java.util.regex.Pattern;


/**
 * Basic implementation of the VideoDao interface.
 *
 * @author Attila Szőke
 */
@Component
public class VideoDaoImpl implements VideoDao {

    private static final String VIDEO_STORE_PATH_PROP = "bingeAtHome.video.store.path";
    private static final String VALID_EXTENSIONS_PROP = "bingeAtHome.video.validExtensions";
    private static final String DEFAULT_VIDEOS_PATH = "./videos";
    private static final String DEFAULT_EXTENSIONS = "mp4,webm";
    private static final String CONFIG_FILE = "./config.properties";

    private String videoStorePath;
    private String[] validExtensions;

    @PostConstruct
    public void init() {
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
            if(videoStorePath == null || videoStorePath.isEmpty()) {
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
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public List<File> findAllVideos() {
        return new ArrayList<>(FileUtils.listFiles(new File(videoStorePath), validExtensions, false));
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public UrlResource findResourceByName(String name) throws MalformedURLException {
        return new UrlResource("file:" + Paths.get(new File(videoStorePath).getAbsolutePath(), name).toString());
    }
}
