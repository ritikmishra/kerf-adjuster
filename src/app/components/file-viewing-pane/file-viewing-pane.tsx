import React, {createRef, RefObject, useEffect, useState} from 'react';
import './file-viewing-pane.scss'
import * as THREE from "three";

interface ThreeJsObjects {
  scene: THREE.Scene,
  camera: THREE.Camera,
  renderer: THREE.Renderer
}

const initThreeJs = (domRef: RefObject<HTMLDivElement>): ThreeJsObjects => {
  // === THREE.JS CODE START ===
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(90, window.innerWidth / window.innerHeight, 0.1, 1000);
  const renderer = new THREE.WebGLRenderer();
  renderer.setSize(window.innerWidth, window.innerHeight);
  if (domRef.current) {
    domRef.current.appendChild(renderer.domElement);
  }

  return {scene, camera, renderer}
  // === THREE.JS EXAMPLE CODE END ===
}

export const FileViewingPane = () => {
  const paneRef = createRef<HTMLDivElement>();
  let [threeJsObjects, setThreeJsObjects] = useState<ThreeJsObjects>();
  let [zoom, setZoom] = useState<number>(5);

  useEffect(() => setThreeJsObjects(initThreeJs(paneRef)), [])
  useEffect(() => {
    if (threeJsObjects) {
      const geometry = new THREE.BoxGeometry(1, 1, 1);
      const material = new THREE.MeshBasicMaterial({color: 0x00ff00});
      const cube = new THREE.Mesh(geometry, material);
      threeJsObjects.scene.add(cube);
      threeJsObjects.camera.position.z = zoom;
      threeJsObjects.renderer.render(threeJsObjects.scene, threeJsObjects.camera);
    }
  }, [threeJsObjects, zoom])
  return (
    <div
      className="pane"
      ref={paneRef}
      onWheel={(event) => {
        setZoom(zoom + (event.deltaY / 10));
        console.log(zoom)
      }}
      onMouseMove={(event) => {
        if (event.buttons === 1) {
          console.log(event.movementX)
        }
      }}
    />
  )
}