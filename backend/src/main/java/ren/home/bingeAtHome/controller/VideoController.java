package ren.home.bingeAtHome.controller;

import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.*;
import org.springframework.web.bind.annotation.*;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.service.VideoService;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.util.List;

/**
 * REST based controller of the application, which serves video related operations.
 *
 * @author Attila Szőke
 */
@RestController
@RequestMapping(path = "/api")
public class VideoController {

    private final VideoService service;

    /**
     * Instantiates a new Video controller.
     *
     * @param service the service
     */
    @Autowired
    public VideoController(VideoService service) {
        this.service = service;
    }

    /**
     * Sends video list as an array of json objects.
     *
     * @return the list of videos
     */
    @Operation(summary = "Gets the list of videos in the store.")
    @ApiResponses(value = {
            @ApiResponse(responseCode = "200", description = "Video list.",
                    content = {@Content(mediaType = "application/json",
                            array = @ArraySchema(schema = @Schema(implementation = Video.class)))})
    })
    @GetMapping("/video")
    public ResponseEntity<List<Video>> listVideos() {
        return ResponseEntity.ok(service.getAllVideos());
    }

    /**
     * Gets video.
     *
     * @param videoName the file name
     * @return the video
     * @throws VideoMissingException the video missing exception
     */
    @Operation(summary = "Gets a video's information.")
    @ApiResponses(value = {
            @ApiResponse(responseCode = "200", description = "Video info.",
                    content = {@Content(mediaType = "application/json", schema = @Schema(implementation = Video.class))}),
            @ApiResponse(responseCode = "404", description = "Video not found.",
                    content = {@Content(mediaType = "application/json")})
    })
    @GetMapping("/video/{videoName}")
    public ResponseEntity<Video> getVideo(@PathVariable String videoName) throws VideoMissingException {
        return ResponseEntity.ok(service.getVideo(videoName));
    }

    /**
     * Streams a video.
     *
     * @param videoName name of the video
     * @param headers   headers of the request, among them the range header is relevant
     * @return ResponseEntity with the requested range of bytes and standard headers
     * @throws VideoMissingException thrown when there is no such video in the video store
     */
    @Operation(summary = "Streams a video.")
    @ApiResponses(value = {
            @ApiResponse(responseCode = "206", description = "Streaming video.",
                    content = {@Content(mediaType = "video/*")}),
            @ApiResponse(responseCode = "404", description = "Video not found.",
                    content = {@Content(mediaType = "application/json")})
    })
    @GetMapping("/stream")
    public ResponseEntity<ResourceRegion> streamVideo(@RequestParam(name = "v") String videoName, @RequestHeader HttpHeaders headers)
            throws VideoMissingException {
        ResourceRegion region = service.prepareContent(videoName, headers);
        return ResponseEntity.status(HttpStatus.PARTIAL_CONTENT)
                .contentType(MediaTypeFactory
                        .getMediaType(region.getResource())
                        .orElse(MediaType.APPLICATION_OCTET_STREAM))
                .body(region);
    }

}
