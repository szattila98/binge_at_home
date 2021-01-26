package ren.home.bingeAtHome.dao.impl;

import org.apache.commons.io.FileUtils;
import org.springframework.core.io.UrlResource;
import org.springframework.stereotype.Component;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.io.IOException;
import java.net.MalformedURLException;
import java.nio.file.InvalidPathException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;


/**
 * Basic implementation of the VideoDao interface.
 *
 * @author Attila Szőke
 */
@Component
public class VideoDaoImpl implements VideoDao {

    /**
     * {@inheritDoc}
     */
    @Override
    public List<File> findAllVideoFiles() {
        return new ArrayList<>(FileUtils.listFiles(new File(ExternalConfig.VIDEO_STORE_PATH), ExternalConfig.VALID_EXTENSIONS, false));
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public File getVideoFile(String fileName) throws IOException {
        File videoFile = new File(ExternalConfig.VIDEO_STORE_PATH, fileName);
        if (!videoFile.exists()) throw new IOException();
        return videoFile;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public UrlResource findResourceByName(String resourceName) throws MalformedURLException, InvalidPathException {
        return new UrlResource("file:" + Paths.get(new File(ExternalConfig.VIDEO_STORE_PATH).getAbsolutePath(), resourceName).toString());
    }

}
