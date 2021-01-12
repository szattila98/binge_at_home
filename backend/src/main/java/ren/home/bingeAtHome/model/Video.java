package ren.home.bingeAtHome.model;


import io.humble.video.Demuxer;
import lombok.*;
import org.apache.commons.io.FilenameUtils;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.attribute.BasicFileAttributes;
import java.util.Date;

/**
 * Represent a streamable video file.
 *
 * @author Attila Szőke
 */
@Getter
@Setter
@NoArgsConstructor
@EqualsAndHashCode
@ToString
public class Video implements Comparable<Video> {

    private String fileName;
    private Date created;
    private Date lastAccessed;
    private long size; // bytes
    private long duration;
    private String extension;
    private Metadata metadata;

    /**
     * Instantiates a new Video without metadata.
     *
     * @param file the file
     * @throws IOException the io exception
     */
    public Video(File file) throws IOException, InterruptedException {
        BasicFileAttributes attr = Files.readAttributes(file.toPath(), BasicFileAttributes.class);
        Demuxer demuxer = Demuxer.make();
        demuxer.open(file.getAbsolutePath(), null, false, true, null, null);

        this.fileName = file.getName();
        this.created = new Date(attr.creationTime().toMillis());
        this.lastAccessed = new Date(attr.lastAccessTime().toMillis());
        this.size = attr.size();
        this.duration = demuxer.getDuration();
        this.extension = FilenameUtils.getExtension(this.fileName);

        demuxer.close();
    }

    /**
     * Instantiates a new Video.
     *
     * @param file     the file
     * @param metadata the metadata
     * @throws IOException the io exception
     */
    public Video(File file, Metadata metadata) throws IOException, InterruptedException {
        this(file);
        this.metadata = metadata;
    }

    @Override
    public int compareTo(Video o) {
        return this.fileName.compareTo(o.fileName);
    }
}
