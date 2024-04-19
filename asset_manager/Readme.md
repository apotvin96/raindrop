# An Manager for Game Assets

## Goals
* Have a single place to access external assets
* Be able to be used as a regular resource with bevy ECS as a Resource
  * https://docs.rs/bevy_ecs/latest/bevy_ecs/
* Have ability to get thread locked mutable access to the asset for external modification
  * Special processing on file
  * Upload mesh/image to GPU and write back vk* Handle
* Referencing assets should externally just look like strings, often just their path
  * Internally this can be something else that is assigned to the string id if optimization can be gained for it

## Nice to Haves
* During dev time, be able to reload assets from disk if they have been updated
* Once the assets is "uploaded" to the GPU, in the case that is relevant, "drop" the data in memory as it is no longer needed
  * Some sort of set of states like...
    * None (No data in ram or uploaded to the GPU)
    * Loaded (Data in ram)
    * Uploaded (Data in GPU, data still in ram)
    * Finished (Data in GPU, data in ram dropped)

## Unknowns
* How do I keep this thread safe?
  * Reference counted thread-safe pointer: Arc?