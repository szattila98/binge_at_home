package ren.home.bingeAtHome.model;


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
    private String extension;
    private Metadata metadata;

    public Video(File file) throws IOException {
        BasicFileAttributes attr = Files.readAttributes(file.toPath(), BasicFileAttributes.class);
        this.fileName = file.getName();
        this.created = new Date(attr.creationTime().toMillis());
        this.lastAccessed = new Date(attr.lastAccessTime().toMillis());
        this.size = attr.size();
        this.extension = FilenameUtils.getExtension(this.fileName);
    }

    public Video(File file, Metadata metadata) throws IOException {
        BasicFileAttributes attr = Files.readAttributes(file.toPath(), BasicFileAttributes.class);
        this.fileName = file.getName();
        this.created = new Date(attr.creationTime().toMillis());
        this.lastAccessed = new Date(attr.lastAccessTime().toMillis());
        this.size = attr.size();
        this.extension = FilenameUtils.getExtension(this.fileName);
        this.metadata = metadata;
    }

    @Override
    public int compareTo(Video o) {
        return this.fileName.compareTo(o.fileName);
    }
}
