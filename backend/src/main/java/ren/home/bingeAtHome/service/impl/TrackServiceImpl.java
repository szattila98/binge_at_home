package ren.home.bingeAtHome.service.impl;

import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import ren.home.bingeAtHome.dao.TrackDao;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.service.TrackService;
import ren.home.bingeAtHome.service.exception.TrackMissingException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.io.File;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

@Slf4j
@Service
public class TrackServiceImpl implements TrackService {

    private final VideoDao videoDao;
    private final TrackDao trackDao;

    /**
     * Instantiates a new Track service.
     *
     * @param videoDao the video dao
     * @param trackDao the track dao
     */
    @Autowired
    public TrackServiceImpl(VideoDao videoDao, TrackDao trackDao) {
        this.videoDao = videoDao;
        this.trackDao = trackDao;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public Map<String, String> getTrackInfo(String videoName) throws VideoMissingException {
        try {
            videoDao.getVideoFile(videoName);
        } catch (IOException e) {
            log.debug("Video with this name does not exist: {}!", videoName);
            throw new VideoMissingException();
        }
        Map<String, String> tracks = new HashMap<>();
        for (File track : trackDao.getTrackFiles(videoName)) {
            String fileName = track.getName();
            String langKey = fileName.substring(fileName.length() - 7);
            langKey = langKey.substring(0, langKey.indexOf("."));
            tracks.put(langKey, fileName);
        }
        log.debug("Track info {} fetched for: {}!", tracks, videoName);
        return tracks;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public File getTrack(String trackName) throws TrackMissingException {
        File track;
        try {
            track = trackDao.readTrack(trackName);
        } catch (IOException e) {
            log.debug("Track like this does not exist: {}!", trackName);
            throw new TrackMissingException();
        }
        log.debug("Track fetched: {}!", trackName);
        return track;
    }
}
