def ConvexHull_py(points):
    
    n = points.shape[0]
    
    centroid = np.mean(points, axis = 0)
    angles = np.zeros((n,1))
    points -= centroid
    
    angles = np.arctan2(points[:,1], points[:,0])
    sort_index = np.argsort(angles)
    
    points = points[sort_index, :]
    angles = angles[sort_index]

    i = 0
    d = 0
    
    while(i < n):
            
        pp_i = (i-1)%n
        pc_i = i
        pn_i = (i+1)%n
        
        pp = points[pp_i,:]
        pc = points[pc_i,:]
        pn = points[pn_i,:]
        
        d = (pc[0]-pp[0])*(pn[1]-pp[1])-(pc[1]-pp[1])*(pn[0]-pp[0])
        
        if(d <= 0):
            points = np.delete(points, i, axis = 0)
            if i != 0:
                i = pp_i - 1
            else :
                i = -1
            n = n - 1
            
        i = i + 1
    
    return points + centroid