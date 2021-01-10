package ren.home.bingeAtHome.controller;

import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import ren.home.bingeAtHome.controller.dto.MetadataInput;
import ren.home.bingeAtHome.service.MetadataService;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;

/**
 * REST based controller of the application, which serves Metadata related operations.
 *
 * @author Attila Szőke
 */
@RestController
@RequestMapping(path = "/api")
public class MetadataController {

    private MetadataService metadataService;

    @Autowired
    public MetadataController(MetadataService metadataService) {
        this.metadataService = metadataService;
    }

    /**
     * Saves the metadata of a video.
     *
     * @param metadataInput the metadata input, containing the film name and the metadata
     */
    @Operation(summary = "Saves the metadata of a video.")
    @ApiResponses(value = {@ApiResponse(responseCode = "200", description = "Metadata saved.")})
    @PostMapping("/metadata")
    public void saveMetadata(MetadataInput metadataInput) throws MetadataCannotBeSavedException {
        metadataService.saveMetadata(metadataInput.getFileName(), metadataInput.getMetadata());
    }
}
