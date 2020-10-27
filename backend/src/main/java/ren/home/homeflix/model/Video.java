package ren.home.homeflix.model;


import com.fasterxml.jackson.annotation.JsonIgnore;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import org.apache.commons.io.FilenameUtils;

import java.util.Date;

/**
 * Represent a streamable video file.
 *
 * @author Attila Szőke
 */
@Getter
@Setter
@NoArgsConstructor
public class Video {

    private String fileName;
    private Date created;
    private Date lastAccessed;
    private long size; // bytes
    private String extension;
    @JsonIgnore
    private String fullPath;

    public Video(String fileName, Date created, Date lastAccessed, long size, String fullPath) {
        this.fileName = fileName;
        this.created = created;
        this.lastAccessed = lastAccessed;
        this.size = size;
        this.extension = FilenameUtils.getExtension(this.fileName);
        this.fullPath = fullPath;
    }
}
