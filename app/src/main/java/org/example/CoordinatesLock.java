package org.example;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.locks.ReentrantLock;
import java.util.concurrent.locks.Condition;

class CoordinatesLock {
    private final ConcurrentHashMap<Coords, ReentrantLock> coordsLocks;
    private final ReentrantLock mainLock;
    private final Condition allLocksAcquired;

    public CoordinatesLock() {
        this.coordsLocks = new ConcurrentHashMap<>();
        this.mainLock = new ReentrantLock();
        this.allLocksAcquired = mainLock.newCondition();
    }


    public List<Coords> lockCoordsList(List<Coords> coordsList) throws InterruptedException {
        List<Coords> lockedCoordsList = new ArrayList<>();
        mainLock.lock();
        try {
            while (true) {
                boolean allLocked = true;
                for (Coords coords : coordsList) {
                    ReentrantLock lock = getCoordsLock(coords);
                    if (!lock.tryLock()) {
                        allLocked = false;
                        break;
                    } else {
                        lockedCoordsList.add(coords);
                    }
                }
                if (allLocked) {
                    allLocksAcquired.signalAll(); // yay!
                    break;
                } else {
                    unlockCoordsList(lockedCoordsList);
                    lockedCoordsList.clear();
                    allLocksAcquired.await(); // retry after suspension
                }
            }
        } finally {
            mainLock.unlock();
        }
        return lockedCoordsList;
    }


    public void unlockCoordsList(List<Coords> coordsList) {
        mainLock.lock();
        try {
            for (Coords coords : coordsList) {
                ReentrantLock coordsLock = getCoordsLock(coords);
                if (coordsLock.isHeldByCurrentThread()) {
                    coordsLock.unlock();
                }
            }
            allLocksAcquired.signalAll();
        } finally {
            mainLock.unlock();
        }
    }

    private ReentrantLock getCoordsLock(Coords coords) {
        return coordsLocks.computeIfAbsent(
                coords,
                k -> new ReentrantLock()
        );
    }

    public List <Coords> getLockedCoords() {
        List <Coords> lockedCoords = new ArrayList<>();
        for (Coords coords : coordsLocks.keySet()) {
            if (coordsLocks.get(coords).isLocked()) {
                lockedCoords.add(coords);
            }
        }
        return lockedCoords;
    }
}