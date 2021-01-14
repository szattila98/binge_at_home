package ren.home.bingeAtHome.controller.validators.annotations;

import ren.home.bingeAtHome.controller.validators.FileNameValidator;

import javax.validation.Constraint;
import javax.validation.Payload;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;

/**
 * Custom validator annotation, infers whether String is a correct file name.
 *
 * @author Attila Szőke
 */
@Constraint(validatedBy = FileNameValidator.class)
@Retention(RetentionPolicy.RUNTIME)
public @interface FileNameConstraint {

    String message() default "the file name should contain an extension at the end, and must be one of the configure ones";

    Class<?>[] groups() default {};

    Class<? extends Payload>[] payload() default {};
}
