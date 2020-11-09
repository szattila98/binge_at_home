package ren.home.bingeAtHome.controller;

import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.HttpHeaders;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.service.VideoService;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import javax.servlet.http.HttpServletRequest;
import java.util.Set;

/**
 * Main REST based controller of the application.
 *
 * @author Attila Szőke
 */
@Slf4j
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
    public Set<Video> listVideos(HttpServletRequest request) {
        log.info("Video list requested. IP: {}", request.getRemoteAddr());
        return service.getAllVideos();
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
                    content = {@Content(mediaType = "video/mp4")}),
            @ApiResponse(responseCode = "404", description = "Video not found.")
    })
    @GetMapping("/video/{videoName}")
    public ResponseEntity<ResourceRegion> streamVideo(HttpServletRequest request, @PathVariable String videoName, @RequestHeader HttpHeaders headers)
            throws VideoMissingException {
        log.info("Video range sent. Video name: {}, Range: {}, IP: {}",
                videoName, headers.getRange(), request.getRemoteAddr());
        return service.prepareContent(videoName, headers);
    }

}
