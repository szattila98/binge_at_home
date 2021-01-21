package ren.home.bingeAtHome.service.exception;

/**
 * Thrown when a track cannot be found.
 *
 * @author Attila Szőke
 */
public class TrackMissingException extends Exception {

    private static final String ERROR_MSG = "Queried track is missing from the track store!";

    public TrackMissingException() {
        super(ERROR_MSG);
    }

    public TrackMissingException(String message) {
        super(message);
    }

    public TrackMissingException(String message, Throwable cause) {
        super(message, cause);
    }

    public TrackMissingException(Throwable cause) {
        super(cause);
    }

    public TrackMissingException(String message, Throwable cause, boolean enableSuppression, boolean writableStackTrace) {
        super(message, cause, enableSuppression, writableStackTrace);
    }
}
