package ren.home.bingeAtHome.dao.impl;

import org.apache.commons.io.FileUtils;
import org.apache.commons.io.filefilter.WildcardFileFilter;
import org.springframework.core.io.UrlResource;
import org.springframework.stereotype.Component;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.util.ExternalConfigurationUtil;

import java.io.File;
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
        return new ArrayList<>(FileUtils.listFiles(new File(ExternalConfigurationUtil.videoStorePath), ExternalConfigurationUtil.validExtensions, false));
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public File getVideoFile(String fileName) {
        return new File(ExternalConfigurationUtil.videoStorePath, fileName);
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public UrlResource findResourceByName(String name) throws MalformedURLException, InvalidPathException {
        return new UrlResource("file:" + Paths.get(new File(ExternalConfigurationUtil.videoStorePath).getAbsolutePath(), name).toString());
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public List<File> getTrackFiles(String videoName) {
        return new ArrayList<>(FileUtils.listFiles(
                new File(ExternalConfigurationUtil.trackStorePath),
                new WildcardFileFilter(videoName + "-*.vtt"),
                null
        ));
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public File readTrack(String trackName) {
        return new File(new File(ExternalConfigurationUtil.trackStorePath).getAbsolutePath(), trackName);
    }
}
