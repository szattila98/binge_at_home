package ren.home.bingeAtHome.model;

import lombok.*;

import java.util.List;

/**
 * Represent the metadata of a video file.
 *
 * @author Attila Szőke
 */
@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
@EqualsAndHashCode
@ToString
public class Metadata {

    private String videoName;
    private String description;
    private List<String> tags;
    private String posterPath;
    private List<String> captionsPath;
}
