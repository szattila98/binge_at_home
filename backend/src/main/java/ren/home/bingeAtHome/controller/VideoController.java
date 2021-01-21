package ren.home.bingeAtHome.controller;

import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.io.FileSystemResource;
import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.HttpHeaders;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.service.VideoService;
import ren.home.bingeAtHome.service.exception.TrackMissingException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.util.List;
import java.util.Map;

/**
 * Main REST based controller of the application.
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
     * @param fileName the file name
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
    @GetMapping("/video/{fileName}")
    public ResponseEntity<Video> getVideo(@PathVariable String fileName) throws VideoMissingException {
        return ResponseEntity.ok(service.getVideo(fileName));
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
        return service.prepareContent(videoName, headers);
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
                    content = {@Content(mediaType = "application/json")})
    })
    @GetMapping("/track/info/{videoName}")
    public ResponseEntity<Map<String, String>> getTrackInfo(@PathVariable String videoName) {
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
