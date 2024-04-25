# A Manager for Game Assets

## Completed
-   Have a single place to access external assets
-   Be able to be used as a regular resource with bevy ECS as a Resource
    -   https://docs.rs/bevy_ecs/latest/bevy_ecs/
-   Have ability to get thread locked mutable access to the asset for external modification 
    -   Special processing on file
    -   Upload mesh/image to GPU and write back vk\* Handle
-   Referencing assets should externally just look like strings, often just their path
-   Once the assets is "uploaded" to the GPU, in the case that is relevant, "drop" the data in memory as it is no longer needed

## TODO
- Add some sort of interface to request a reload of an asset
    - Currently, this can be done but removing the mesh from the hashmap, but that sucks
- Using the reload request, can I tell the manager to automatically reload an asset if it sees a local filesystem change?
- Add a single thread that the asset manager runs on to perform the loads and processing
- Add a separate thread for the renderer that performs gpu uploads of meshes from the asset manager, can i just spawn another thread for each upload that is needed? i doubt it