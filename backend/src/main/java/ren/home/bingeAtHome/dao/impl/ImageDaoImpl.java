package ren.home.bingeAtHome.dao.impl;

import org.springframework.stereotype.Component;
import ren.home.bingeAtHome.dao.ImageDao;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.io.IOException;

/**
 * Basic implementation of the ImageDao interface.
 *
 * @author Attila Szőke
 */
@Component
public class ImageDaoImpl implements ImageDao {

    /**
     * {@inheritDoc}
     *
     * @return
     */
    @Override
    public File readImage(String imageName) throws IOException {
        File image = new File(ExternalConfig.IMAGE_STORE_PATH, imageName);
        if (!image.exists()) throw new IOException();
        return image;
    }
}
