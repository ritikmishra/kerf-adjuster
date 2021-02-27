import React, {createRef, FC, ReactElement, RefObject, useEffect, useState} from "react";
import styles from "./dxf-display-area.module.scss";
import * as THREE from "three";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls";

class ThreeJsObjects {
    constructor(
        public readonly scene: THREE.Scene,
        public readonly camera: THREE.OrthographicCamera,
        public readonly renderer: THREE.Renderer,
        public readonly plane: THREE.Plane,
        public readonly width: number,
        public readonly height: number,
        public readonly orbitControls: OrbitControls) {
    }

    render(): void {
        this.renderer.render(this.scene, this.camera);
    }
}

const scaleFactor = 100;

const initThreeJs = (domRef: RefObject<HTMLDivElement>): ThreeJsObjects => {
    // === THREE.JS CODE START ===
    const plane = new THREE.Plane(new THREE.Vector3(0, 0, 1)); // xy-plane
    const scene = new THREE.Scene();
    const height = window.innerHeight;
    const width = window.innerWidth;

    const camera = new THREE.OrthographicCamera(-width / scaleFactor, width / scaleFactor, height / scaleFactor, -height / scaleFactor, 0.1, 1000);
    camera.position.z = 1;
    const renderer = new THREE.WebGLRenderer();
    renderer.setSize(width, height);
    if (domRef.current) {
        domRef.current.appendChild(renderer.domElement);
    }

    const orbitControls = new OrbitControls(camera, domRef.current!);
    orbitControls.enableRotate = false;
    orbitControls.mouseButtons = {
        LEFT: THREE.MOUSE.PAN,
        MIDDLE: THREE.MOUSE.DOLLY,
        RIGHT: THREE.MOUSE.ROTATE, // a noop
    };


    const ret = new ThreeJsObjects(scene, camera, renderer, plane, width, height, orbitControls);

    function animate() {
        requestAnimationFrame(animate);
        ret.render();
    }

    animate();

    return ret;
    // === THREE.JS EXAMPLE CODE END ===
};

interface FileViewingPaneProps {
    fileBuffer: Buffer
}

export const DxfDisplayArea: FC<FileViewingPaneProps> = (props): ReactElement => {
    const paneRef = createRef<HTMLDivElement>();
    const [threeJsObjects, setThreeJsObjects] = useState<ThreeJsObjects>();

    // Initialize everything
    useEffect(() => {
        setThreeJsObjects(initThreeJs(paneRef));
    }, []);


    // Add the green box (TODO: delete)
    useEffect(() => {
        if (threeJsObjects) {
            const geometry = new THREE.BoxGeometry(90, 1, 1);
            geometry.translate(6, 3, 0);

            const geometry2 = new THREE.BoxGeometry(1, 1, 1);
            const material = new THREE.MeshBasicMaterial({color: 0x00ff00});
            const cube = new THREE.Mesh(geometry, material);
            threeJsObjects.scene.add(cube, new THREE.Mesh(geometry2, material));
            threeJsObjects.render();
        }
    }, [threeJsObjects]);

    // Fix scene when window is resized
    useEffect(function handleResize() {
        const width = window.innerWidth;
        const height = window.innerHeight;
        if (threeJsObjects) {
            threeJsObjects.renderer.setSize(width, height);
            threeJsObjects.camera.left = -width / scaleFactor;
            threeJsObjects.camera.right = width / scaleFactor;
            threeJsObjects.camera.top = height / scaleFactor;
            threeJsObjects.camera.bottom = -height / scaleFactor;
            threeJsObjects.camera.updateProjectionMatrix();
        }
        window.addEventListener("resize", handleResize);
    });

    return (
        <div
            className={styles.pane}
            ref={paneRef}
        />
    );
};
