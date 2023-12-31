use bevy::math::{Vec2, Vec3, Vec4};
use serde::{Deserialize, Serialize};

/// An enumeration of the various coordinate systems used in the graphics rendering pipeline.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "coordinate")]
pub enum Coordinate {
    /// World Coordinates represent points in the 3D world space.
    /// This is where objects have their original sizes and positions
    /// before any transformations are applied.
    ///
    /// - Parameters:
    ///   - x: The x-coordinate in world space.
    ///   - y: The y-coordinate in world space.
    ///   - z: The z-coordinate in world space.
    WorldSpaceCoord(Vec3),

    /// View Coordinates are the result of applying the view transformation to the world coordinates.
    /// This transformation typically involves translating and rotating the scene to the camera's
    /// viewpoint, effectively repositioning the entire world in relation to the camera's position
    /// and orientation.
    ///
    /// - Parameters:
    ///   - x: The x-coordinate in view space.
    ///   - y: The y-coordinate in view space.
    ///   - z: The z-coordinate in view space.
    ViewSpaceCoord(Vec3),

    /// Clip Coordinates are the result of applying a projection transformation to the view coordinates.
    /// This transformation projects the 3D scene onto a 2D plane and defines what will be visible on
    /// the screen. The w component is used for perspective division and is essential for perspective
    /// projection.
    ///
    /// - Parameters:
    ///   - x: The x-coordinate in clip space.
    ///   - y: The y-coordinate in clip space.
    ///   - z: The z-coordinate in clip space.
    ///   - w: The homogeneous coordinate used for perspective division.
    ClipSpaceCoord(Vec4),

    /// Normalized Device Coordinates (NDC) are obtained by dividing the clip coordinates by their w component.
    /// After this perspective division, the coordinates are in a unit cube where the range is from -1 to 1
    /// in each axis. The NDC determine where an object appears in the viewport and handles the aspect ratio
    /// of the output device.
    ///
    /// - Parameters:
    ///   - x: The x-coordinate in NDC space.
    ///   - y: The y-coordinate in NDC space.
    ///   - z: The z-coordinate in NDC space.
    NormalizedDeviceSpaceCoord(Vec3),

    /// Screen Coordinates are the final transformation from NDC and are specific to the output device,
    /// typically a computer screen or a window. This transformation involves scaling the NDCs to the
    /// viewport's dimensions and converting them to pixel coordinates. These are the actual coordinates
    /// used to position elements on the screen.
    ///
    /// - Parameters:
    ///   - x: The x-coordinate on the screen in pixels.
    ///   - y: The y-coordinate on the screen in pixels.
    /// The origin is at the bottom left, unlike for svg coordiante system
    ScreenSpaceCoord(Vec2),
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn serialisation() -> () {
        // Sample coordinate values

        let j = r#"
        {
            "type": "WorldSpaceCoord",
            "coordinate": [1.0,2.0,3.0]
        }"#;

        // Serialize to JSON
        let world_json: Coordinate = serde_json::from_str(j).unwrap();

        print!("{:?}", &world_json);
    }
}
