package ren.home.bingeAtHome.controller.dto;

import lombok.*;

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.Size;
import java.util.List;

/**
 * Data access object for Metadata input.
 *
 * @author Attila Szőke
 */
@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
@EqualsAndHashCode
@ToString
public class MetadataInput {

    @NotBlank(message = "file name is mandatory")
    @Size(max = 100, message = "file name max is 100 characters long")
    private String fileName;
    @NotBlank(message = "video name is mandatory")
    @Size(min = 2, max = 100, message = "video name min is 2 and max is 100 characters long")
    private String videoName;
    @NotBlank(message = "description is mandatory")
    @Size(min = 1, max = 2000, message = "description min is 100 and max is 2000 characters long")
    private String description;
    @Size(max = 10, message = "tags min count is 1, max is 10")
    private List<String> tags;
    
}
