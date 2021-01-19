package ren.home.bingeAtHome.service.impl;

import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import ren.home.bingeAtHome.dao.ImageDao;
import ren.home.bingeAtHome.service.ImageService;
import ren.home.bingeAtHome.service.exception.ImageMissingException;

import java.io.IOException;
import java.nio.file.Path;

/**
 * Base implementation of VideoService.
 *
 * @author Attila Szőke
 */
@Slf4j
@Service
public class ImageServiceImpl implements ImageService {

    private ImageDao imageDao;

    /**
     * Instantiates a new Image service.
     *
     * @param imageDao the image dao
     */
    @Autowired
    public ImageServiceImpl(ImageDao imageDao) {
        this.imageDao = imageDao;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public Path getPosterImage(String videoFileName) throws ImageMissingException {
        try {
            log.debug("Reading image for video: {}!", videoFileName);
            return imageDao.readImage(videoFileName + ".webp");
        } catch (IOException e) {
            throw new ImageMissingException();
        }
    }
}
