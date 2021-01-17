package ren.home.bingeAtHome.dao.impl;

import org.springframework.core.io.UrlResource;
import org.springframework.stereotype.Component;
import ren.home.bingeAtHome.dao.ImageDao;
import ren.home.bingeAtHome.util.ExternalConfigurationUtil;

import java.io.File;
import java.io.IOException;
import java.nio.file.Paths;

/**
 * Basic implementation of the ImageDao interface.
 *
 * @author Attila Szőke
 */
@Component
public class ImageDaoImpl implements ImageDao {

    /**
     * {@inheritDoc}
     */
    @Override
    public UrlResource readImage(String imageId) throws IOException {
        UrlResource image = new UrlResource("file:" + Paths.get(new File(ExternalConfigurationUtil.imageStorePath).getAbsolutePath(), imageId).toString());
        if (!image.exists()) throw new IOException();
        return image;
    }
}
