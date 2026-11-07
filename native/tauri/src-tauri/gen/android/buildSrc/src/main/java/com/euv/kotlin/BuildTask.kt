import java.io.File
import org.gradle.api.DefaultTask
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.TaskAction

open class BuildTask : DefaultTask() {
    @Input var rootDirRel: String? = null
    @Input var target: String? = null
    @Input var release: Boolean? = null

    @TaskAction
    fun assemble() {
        project.logger.info("Skipping Rust build (pre-compiled): target=" + target + " release=" + release)
    }
}
