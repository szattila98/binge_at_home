package ren.home.bingeAtHome.dao.impl;

import org.apache.commons.io.FileUtils;
import org.apache.commons.io.filefilter.WildcardFileFilter;
import org.springframework.stereotype.Component;
import ren.home.bingeAtHome.dao.TrackDao;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

@Component
public class TrackDaoImpl implements TrackDao {

    private static final String TRACK_REGEX = "-*.vtt";

    /**
     * {@inheritDoc}
     */
    @Override
    public List<File> getTrackFiles(String videoName) {
        return new ArrayList<>(FileUtils.listFiles(
                new File(ExternalConfig.TRACK_STORE_PATH),
                new WildcardFileFilter(videoName + TRACK_REGEX),
                null
        ));
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public File readTrack(String trackName) throws IOException {
        File track = new File(ExternalConfig.TRACK_STORE_PATH, trackName);
        if (!track.exists()) throw new IOException();
        return track;
    }
}
