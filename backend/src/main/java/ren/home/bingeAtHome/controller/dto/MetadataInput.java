package ren.home.bingeAtHome.controller.dto;

import lombok.*;
import ren.home.bingeAtHome.model.Metadata;

/**
 * Data access object for Metadata input.
 *
 * @author Attila Szőke
 */
@Getter
@Setter
@NoArgsConstructor
@EqualsAndHashCode
@ToString
public class MetadataInput {

    private String fileName;
    private Metadata metadata;
}
