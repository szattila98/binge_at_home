package ren.home.bingeAtHome.controller.dto;

import lombok.Getter;
import lombok.Setter;
import ren.home.bingeAtHome.model.Metadata;

/**
 * Data access object for Metadata input.
 *
 * @author Attila Szőke
 */
@Getter
@Setter
public class MetadataInput {

    private String fileName;
    private Metadata metadata;
}
