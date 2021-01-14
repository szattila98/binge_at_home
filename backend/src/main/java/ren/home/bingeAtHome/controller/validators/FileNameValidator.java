package ren.home.bingeAtHome.controller.validators;

import ren.home.bingeAtHome.controller.validators.annotations.FileNameConstraint;
import ren.home.bingeAtHome.dao.ExternalConfigurationUtil;

import javax.validation.ConstraintValidator;
import javax.validation.ConstraintValidatorContext;

/**
 * Validator class for FileNameConstraint annotation.
 *
 * @author Attila Szőke
 */
public class FileNameValidator implements ConstraintValidator<FileNameConstraint, String> {

    @Override
    public void initialize(FileNameConstraint constraintAnnotation) {
    }

    @Override
    public boolean isValid(String value, ConstraintValidatorContext context) {
        for (String ext : ExternalConfigurationUtil.validExtensions) {
            if (value.endsWith("." + ext))
                return true;
        }
        return false;
    }
}
