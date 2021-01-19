package ren.home.bingeAtHome.controller;

import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.io.FileSystemResource;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import ren.home.bingeAtHome.service.ImageService;
import ren.home.bingeAtHome.service.exception.ImageMissingException;

/**
 * REST based controller of the application, which serves Image related operations.
 *
 * @author Attila Szőke
 */
@RestController
@RequestMapping(path = "/api")
public class ImageController {

    private ImageService service;

    /**
     * Instantiates a new Image controller.
     *
     * @param service the service
     */
    @Autowired
    public ImageController(ImageService service) {
        this.service = service;
    }

    /**
     * Gets a poster image.
     *
     * @param videoFileName the video file name
     * @return the poster
     * @throws ImageMissingException the image missing exception
     */
    @Operation(summary = "Gets a poster image.")
    @ApiResponses(value = {
            @ApiResponse(responseCode = "200", description = "Image exists.", content = {@Content(mediaType = "image/webm")}),
            @ApiResponse(responseCode = "404", description = "Image not found!", content = {@Content(mediaType = "application/json")})
    })
    @GetMapping("/poster/{videoFileName}")
    public ResponseEntity<FileSystemResource> getPoster(@PathVariable String videoFileName) throws ImageMissingException {
        return ResponseEntity.ok(new FileSystemResource(service.getPosterImage(videoFileName)));
    }
}
