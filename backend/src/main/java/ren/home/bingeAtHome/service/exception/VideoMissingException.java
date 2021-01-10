package ren.home.bingeAtHome.service.exception;

/**
 * Thrown when a Video is missing or cannot be reached.
 *
 * @author Attila Szőke
 */
public class VideoMissingException extends Exception {

    private static final String ERROR_MSG = "Queried video is missing from the video store!";

    public VideoMissingException() {
        super(ERROR_MSG);
    }

    public VideoMissingException(String message) {
        super(message);
    }

    public VideoMissingException(String message, Throwable cause) {
        super(message, cause);
    }

    public VideoMissingException(Throwable cause) {
        super(cause);
    }

    public VideoMissingException(String message, Throwable cause, boolean enableSuppression, boolean writableStackTrace) {
        super(message, cause, enableSuppression, writableStackTrace);
    }
}
