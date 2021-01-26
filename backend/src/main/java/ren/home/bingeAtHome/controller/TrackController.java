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
import ren.home.bingeAtHome.service.TrackService;
import ren.home.bingeAtHome.service.exception.TrackMissingException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.util.Map;

/**
 * REST based controller of the application, which serves Track related operations.
 *
 * @author Attila Szőke
 */
@RestController
@RequestMapping(path = "/api")
public class TrackController {

    private final TrackService service;

    /**
     * Instantiates a new Track controller.
     *
     * @param trackService the track service
     */
    @Autowired
    public TrackController(TrackService trackService) {
        this.service = trackService;
    }

    /**
     * Gets the track information for a video.
     *
     * @param videoName the video name
     * @return the track info
     */
    @Operation(summary = "Gets the track information for a video.")
    @ApiResponses(value = {
            @ApiResponse(responseCode = "200", description = "Track info.",
                    content = {@Content(mediaType = "application/json")}),
            @ApiResponse(responseCode = "404", description = "Video not found.",
                    content = {@Content(mediaType = "application/json")})
    })
    @GetMapping("/track/info/{videoName}")
    public ResponseEntity<Map<String, String>> getTrackInfo(@PathVariable String videoName) throws VideoMissingException {
        return ResponseEntity.ok(service.getTrackInfo(videoName));
    }

    /**
     * Gets a track from the track store.
     *
     * @param trackName the track name
     * @return the track
     */
    @Operation(summary = "Gets a track from the track store.")
    @ApiResponses(value = {
            @ApiResponse(responseCode = "200", description = "Track.",
                    content = {@Content(mediaType = "text/vtt")})
    })
    @GetMapping("/track/{trackName}")
    public ResponseEntity<FileSystemResource> getTrack(@PathVariable String trackName) throws TrackMissingException {
        return ResponseEntity.ok(new FileSystemResource(service.getTrack(trackName)));
    }
}
