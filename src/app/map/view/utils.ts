import * as THREE from "three";


function clearRecursive(object: THREE.Object3D) {
    if (object.type === "Line") {
        (object as THREE.Line).geometry.dispose();
        ((object as THREE.Line).material as THREE.Material).dispose();
    } else if (object.type === "Mesh") {
        (object as THREE.Mesh).geometry.dispose();
        ((object as THREE.Mesh).material as THREE.Material).dispose();
    }
    object.children.forEach((item: THREE.Object3D) => {
        clearRecursive(item)
    });
    object.clear()
}

function getChildrenCount(obj: THREE.Object3D) {
    let count = 0
    obj.traverse((child: THREE.Object3D) => {
        count++
    })
    return count
}

export { getChildrenCount, clearRecursive };