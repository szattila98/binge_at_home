package ren.home.bingeAtHome.controller.dto;

import lombok.*;
import ren.home.bingeAtHome.controller.validators.annotations.FileNameConstraint;
import ren.home.bingeAtHome.model.Metadata;

import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.Size;

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

    @NotBlank(message = "file name is mandatory")
    @Size(max = 100, message = "file name max is 100 characters long")
    @FileNameConstraint
    private String fileName;
    @Valid
    private Metadata metadata;
}
