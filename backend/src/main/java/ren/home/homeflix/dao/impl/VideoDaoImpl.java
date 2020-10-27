package ren.home.homeflix.dao.impl;

import org.apache.commons.io.FileUtils;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;
import ren.home.homeflix.dao.VideoDao;

import java.io.File;
import java.util.ArrayList;
import java.util.List;


/**
 * Basic implementation of the VideoDao interface.
 *
 * @author Attila Szőke
 */
@Component
public class VideoDaoImpl implements VideoDao {

    @Value("${homeflix.video.store.root}")
    private String videoStoreRoot;
    @Value("#{'${homeflix.video.validExtensions}'.split(',')}")
    private String[] validExtensions;

    /**
     * {@inheritDoc}
     */
    @Override
    public List<File> findAllVideos() {
        return new ArrayList<>(FileUtils.listFiles(new File(videoStoreRoot), validExtensions, false));
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public File findByName(String name) {
        return FileUtils.getFile(new File(videoStoreRoot), name);
    }
}
