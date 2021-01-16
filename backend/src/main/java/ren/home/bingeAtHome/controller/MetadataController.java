package ren.home.bingeAtHome.controller;

import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import ren.home.bingeAtHome.controller.dto.MetadataInput;
import ren.home.bingeAtHome.service.MetadataService;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import javax.validation.Valid;

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
    @ApiResponses(value = {
            @ApiResponse(responseCode = "200", description = "Metadata saved.", content = {@Content(mediaType = "text/plain")}),
            @ApiResponse(responseCode = "404", description = "Video not found!", content = {@Content(mediaType = "application/json")}),
            @ApiResponse(responseCode = "400", description = "Metadata input invalid!", content = {@Content(mediaType = "application/json")}),
            @ApiResponse(responseCode = "500", description = "Metadata could not be saved!")
    })
    @PostMapping("/metadata")
    public ResponseEntity<String> saveMetadata(@Valid @RequestBody MetadataInput metadataInput) throws MetadataCannotBeSavedException, VideoMissingException {
        return ResponseEntity.ok(metadataService.saveMetadata(metadataInput.getFileName(), metadataInput.getMetadata()));
    }
}
